use libucl_bind::*;
use utils;

use super::Object;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Emitter {
    JSON,
    JSONCompact,
    Config,
    YAML,
    MsgPack,
    Max,
}

impl Emitter {
    pub fn emit<T: AsRef<Object>>(&self, obj: T) -> Option<String> {
        let emit = unsafe { ucl_object_emit(obj.as_ref().obj, Into::into(*self)) };
        utils::to_str(emit)
    }
}

impl From<ucl_emitter_t> for Emitter {
    fn from(raw: ucl_emitter_t) -> Self {
        match raw {
            ucl_emitter_t::UCL_EMIT_JSON => Emitter::JSON,
            ucl_emitter_t::UCL_EMIT_JSON_COMPACT => Emitter::JSONCompact,
            ucl_emitter_t::UCL_EMIT_CONFIG => Emitter::Config,
            ucl_emitter_t::UCL_EMIT_YAML => Emitter::YAML,
            ucl_emitter_t::UCL_EMIT_MSGPACK => Emitter::MsgPack,
            ucl_emitter_t::UCL_EMIT_MAX => Emitter::Max
        }
    }
}

impl Into<ucl_emitter_t> for Emitter {
    fn into(self) -> ucl_emitter_t {
        match self {
            Emitter::JSON => ucl_emitter_t::UCL_EMIT_JSON,
            Emitter::JSONCompact => ucl_emitter_t::UCL_EMIT_JSON_COMPACT,
            Emitter::Config => ucl_emitter_t::UCL_EMIT_CONFIG,
            Emitter::YAML => ucl_emitter_t::UCL_EMIT_YAML,
            Emitter::MsgPack => ucl_emitter_t::UCL_EMIT_MSGPACK,
            Emitter::Max => ucl_emitter_t::UCL_EMIT_MAX
        }
    }
}
