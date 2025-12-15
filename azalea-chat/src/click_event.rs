use serde::{Deserialize, Serialize};
#[cfg(feature = "simdnbt")]
use simdnbt::owned::{NbtCompound, NbtTag};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
pub enum ClickEvent {
    OpenUrl {
        #[serde(rename = "value", alias = "url")]
        url: String,
    },
    OpenFile {
        #[serde(rename = "value", alias = "path")]
        path: String,
    },
    RunCommand {
        #[serde(rename = "value", alias = "command")]
        command: String,
    },
    SuggestCommand {
        #[serde(rename = "value", alias = "command")]
        command: String,
    },
    // TODO: this uses Dialog.CODEC
    ShowDialog,
    ChangePage {
        #[serde(rename = "value", alias = "page")]
        page: i32,
    },
    CopyToClipboard {
        value: String,
    },
    Custom {
        id: String,
        #[cfg(feature = "simdnbt")]
        // payload: Nbt, // Deserialize not implemented for Nbt
        #[serde(skip)]
        _payload: (),
    },
}

#[cfg(feature = "simdnbt")]
impl simdnbt::Serialize for ClickEvent {
    fn to_compound(self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        let mut action = |s: &str| {
            compound.insert("action", s);
        };
        match self {
            ClickEvent::OpenUrl { url } => {
                action("open_url");
                compound.insert("url", url);
            }
            ClickEvent::OpenFile { path } => {
                action("open_file");
                compound.insert("path", path);
            }
            ClickEvent::RunCommand { command } => {
                action("run_command");
                compound.insert("command", command);
            }
            ClickEvent::SuggestCommand { command } => {
                action("suggest_command");
                compound.insert("command", command);
            }
            ClickEvent::ShowDialog => {
                action("show_dialog");
            }
            ClickEvent::ChangePage { page } => {
                action("change_page");
                compound.insert("page", NbtTag::Int(page));
            }
            ClickEvent::CopyToClipboard { value } => {
                action("copy_to_clipboard");
                compound.insert("value", value);
            }
            ClickEvent::Custom { id, .. } => {
                action("custom");
                compound.insert("id", id);
                // compound.insert("payload", (**payload).clone());
            }
        }
        compound
    }
}

#[cfg(feature = "simdnbt")]
impl simdnbt::Deserialize for ClickEvent {
    fn from_compound(
        compound: simdnbt::borrow::NbtCompound,
    ) -> Result<Self, simdnbt::DeserializeError> {
        let action = compound
            .string("action")
            .ok_or(simdnbt::DeserializeError::MissingField)?
            .to_string();

        match action.as_str() {
            "open_url" => Ok(ClickEvent::OpenUrl {
                url: compound
                    .string("value")
                    .or(compound.string("url"))
                    .ok_or(simdnbt::DeserializeError::MissingField)?
                    .to_string(),
            }),
            "open_file" => Ok(ClickEvent::OpenFile {
                path: compound
                    .string("value")
                    .or(compound.string("path"))
                    .ok_or(simdnbt::DeserializeError::MissingField)?
                    .to_string(),
            }),
            "run_command" => Ok(ClickEvent::RunCommand {
                command: compound
                    .string("value")
                    .or(compound.string("command"))
                    .ok_or(simdnbt::DeserializeError::MissingField)?
                    .to_string(),
            }),
            "suggest_command" => Ok(ClickEvent::SuggestCommand {
                command: compound
                    .string("value")
                    .or(compound.string("command"))
                    .ok_or(simdnbt::DeserializeError::MissingField)?
                    .to_string(),
            }),
            "show_dialog" => Ok(ClickEvent::ShowDialog),
            "change_page" => Ok(ClickEvent::ChangePage {
                page: compound
                    .int("value")
                    .or(compound.int("page"))
                    .ok_or(simdnbt::DeserializeError::MissingField)?,
            }),
            "copy_to_clipboard" => Ok(ClickEvent::CopyToClipboard {
                value: compound
                    .string("value")
                    .ok_or(simdnbt::DeserializeError::MissingField)?
                    .to_string(),
            }),
            "custom" => Ok(ClickEvent::Custom {
                id: compound
                    .string("id")
                    .ok_or(simdnbt::DeserializeError::MissingField)?
                    .to_string(),
                _payload: (),
            }),
            _ => Err(simdnbt::DeserializeError::MissingField),
        }
    }
}
