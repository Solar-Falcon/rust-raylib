use serde_json::Value;

#[derive(Clone, Debug, serde::Deserialize)]
struct Definition {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    value: Value,
    #[allow(dead_code)]
    description: String,
}

impl Definition {
    fn generate_code(&self, code: &mut String) {
        match self.kind.as_str() {
            "INT" => {
                code.push_str(&format!(
                    "pub const {}: u32 = {};\n",
                    self.name,
                    self.value.as_u64().unwrap()
                ));
            }
            "STRING" => {
                code.push_str(&format!(
                    "pub const {}: &str = \"{}\";\n",
                    self.name,
                    self.value.as_str().unwrap()
                ));
            }
            _ => {} // sorry colors, I'll add you someday
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Struct {
    name: String,
    description: String,
    fields: Vec<TypedIdent>,
}

impl Struct {
    fn generate_code(&self, code: &mut String) {
        code.push('\n');
        code.push_str(&format!("/// {}\n", self.description));
        code.push_str("#[repr(C)]\n");
        code.push_str("#[derive(Clone, Debug)]\n");

        code.push_str(&format!("pub struct {} {{\n", self.name));

        for field in self.fields.iter() {
            code.push_str(&format!("\t/// {}\n", field.description));
            code.push_str(&format!(
                "\tpub {}: {},\n",
                field.name,
                format_type(field.data_type.as_str())
            ));
        }

        code.push_str("}\n");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RefType {
    None,
    Ref,
    DoubleRef,
    Array(u32),
}

#[derive(Clone, Debug, serde::Deserialize)]
struct TypedIdent {
    name: String,
    #[serde(rename = "type")]
    data_type: String,
    #[serde(default)]
    description: String,
}

fn format_type(data_type: &str) -> String {
    use RefType::*;

    let (dt, is_const) = if let Some(dt) = data_type.strip_prefix("const ") {
        (dt, true)
    } else {
        (data_type, false)
    };

    let (dt, ref_type) = if let Some(new_dt) = dt.strip_suffix(" *") {
        (new_dt, Ref)
    } else if let Some(new_dt) = dt.strip_suffix(" **") {
        (new_dt, DoubleRef)
    } else if let Some(pos1) = dt.find('[') {
        (
            &dt[..pos1],
            Array(dt[(pos1 + 1)..(dt.find(']').unwrap())].parse().unwrap()),
        )
    } else {
        (dt, None)
    };

    let type_str = match dt {
        "float" => "core::ffi::c_float",
        "double" => "core::ffi::c_double",
        "unsigned char" => "core::ffi::c_uchar",
        "signed char" => "core::ffi::c_schar",
        "char" => "core::ffi::c_char",
        "unsigned short" => "core::ffi::c_ushort",
        "signed short" | "short" => "core::ffi::c_short",
        "unsigned int" => "core::ffi::c_uint",
        "signed int" | "int" => "core::ffi::c_int",
        "unsigned long" => "core::ffi::c_ulong",
        "signed long" | "long" => "core::ffi::c_long",
        "unsigned long long" => "core::ffi::c_ulonglong",
        "signed long long" | "long long" => "core::ffi::c_longlong",
        "void" => "core::ffi::c_void",
        _ => dt,
    };

    let mutability = if is_const { "const" } else { "mut" };

    match ref_type {
        None => type_str.to_string(),
        Ref => format!("*{} {}", mutability, type_str),
        DoubleRef => format!("*{} *{} {}", mutability, mutability, type_str),
        Array(size) => format!("[{}; {}]", type_str, size),
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Enum {
    name: String,
    description: String,
    values: Vec<EnumValue>,
}

impl Enum {
    fn prefix_count(enum_name: &str) -> usize {
        match enum_name {
            "CubemapLayout"
            | "GamepadAxis"
            | "GamepadButton"
            | "MaterialMapIndex"
            | "MouseButton"
            | "MouseCursor"
            | "PixelFormat"
            | "ShaderAttributeDataType"
            | "ShaderLocationIndex"
            | "ShaderUniformDataType"
            | "TextureFilter"
            | "TextureWrap" => 2,
            _ => 1,
        }
    }

    fn format_value_name(enum_name: &str, value_name: &str) -> String {
        let skips = Self::prefix_count(enum_name);
        let parts = value_name.split('_').skip(skips);

        parts
            .map(|s| {
                let mut s = s.to_string();

                if s.len() > 1
                    && !(enum_name == "PixelFormat" && s.contains(|c: char| c.is_ascii_digit()))
                {
                    s[1..].make_ascii_lowercase();
                }

                s
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn generate_code(&self, code: &mut String) {
        code.push('\n');
        code.push_str(&format!("/// {}\n", self.description));
        code.push_str("#[repr(C)]\n");
        code.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n");
        code.push_str(
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n",
        );

        code.push_str(&format!("pub enum {} {{\n", self.name));

        let mut values = fnv::FnvHashSet::default();

        for value in self.values.iter() {
            if !values.contains(&value.value) {
                values.insert(value.value);

                code.push_str(&format!("\t/// {}\n", value.description));
                code.push_str(&format!(
                    "\t{} = {},\n",
                    Self::format_value_name(&self.name, &value.name),
                    value.value
                ));
            }
            // otherwise, sadly, ignore it
        }

        code.push_str("}\n");
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
struct EnumValue {
    name: String,
    description: String,
    value: u32,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Function {
    name: String,
    description: String,
    #[serde(rename = "returnType")]
    return_type: String,
    #[serde(default)]
    params: Vec<TypedIdent>,
}

impl Function {
    fn generate_code_as_callback(&self, code: &mut String) {
        code.push_str(&format!("/// {}\n", self.description));
        code.push_str(&format!(
            "pub type {} = Option<unsafe extern \"C\" fn",
            self.name
        ));

        self.generate_code_common(code);

        code.push_str(">;\n");
    }

    fn generate_code_as_function(&self, code: &mut String) {
        code.push_str(&format!("/// {}\n", self.description));
        code.push_str(&format!("pub unsafe extern \"C\" fn {}", self.name));

        self.generate_code_common(code);

        code.push_str(";\n");
    }

    fn generate_code_common(&self, code: &mut String) {
        code.push('(');

        let name = if self.name == "type" {
            "r#type"
        } else if self.name == "box" {
            "r#box"
        } else {
            self.name.as_str()
        };

        for param in self.params.iter() {
            if param.data_type == "..." {
                code.push_str("..., ");
            } else {
                code.push_str(&format!(
                    "{}: {}, ",
                    name,
                    format_type(param.data_type.as_str())
                ));
            }
        }

        code.push(')');

        if self.return_type != "void" {
            code.push_str(&format!(" -> {}", format_type(self.return_type.as_str())));
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Api {
    defines: Vec<Definition>,
    structs: Vec<Struct>,
    aliases: Vec<TypedIdent>,
    enums: Vec<Enum>,
    callbacks: Vec<Function>,
    functions: Vec<Function>,
}

impl Api {
    pub fn generate_code(&self) -> String {
        let mut code = String::new();

        // Aren't included in raylib.h
        code.push_str("pub const MAX_SHADER_LOCATIONS: usize = 32;\n");
        code.push_str("pub const MAX_MATERIAL_MAPS: usize = 12;\n");
        code.push_str(
            "#[repr(C)]\npub struct rAudioBuffer { _empty: core::marker::PhantomData<()> }\n",
        );
        code.push_str(
            "#[repr(C)]\npub struct rAudioProcessor { _empty: core::marker::PhantomData<()> }\n",
        );

        for define in self.defines.iter() {
            define.generate_code(&mut code);
        }

        for struc in self.structs.iter() {
            struc.generate_code(&mut code);
        }

        for alias in self.aliases.iter() {
            code.push('\n');
            code.push_str(&format!("/// {}\n", alias.description));
            code.push_str(&format!(
                "pub type {} = {};\n",
                alias.name,
                format_type(alias.data_type.as_str())
            ));
        }

        for enu in self.enums.iter() {
            enu.generate_code(&mut code);
        }

        for cb in self.callbacks.iter() {
            cb.generate_code_as_callback(&mut code);
        }

        for func in self.functions.iter() {
            func.generate_code_as_function(&mut code);
        }

        code
    }
}
