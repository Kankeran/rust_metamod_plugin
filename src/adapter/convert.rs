use crate::{adapter::api, metamod::meta_util};

pub fn convert_result(result: api::Return) -> i32 {
    match result {
        api::Return::Ignored => meta_util::RESULT_IGNORED,
        api::Return::Handled => meta_util::RESULT_HANDLED,
        api::Return::HandledMain => meta_util::RESULT_SUPERCEDE,
        api::Return::Override => meta_util::RESULT_OVERRIDE,
        api::Return::Supercede => meta_util::RESULT_SUPERCEDE,
    }
}
