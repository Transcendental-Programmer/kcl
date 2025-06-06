//! Copyright The KCL Authors. All rights reserved.
use crate::*;

pub const YAML_STREAM_SEP: &str = "\n---\n";
pub const JSON_STREAM_SEP: &str = "\n";

/// encode(data, sort_keys=False, ignore_private=False, ignore_none=False)
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_yaml_encode(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);

    if let Some(arg0) = get_call_arg(args, kwargs, 0, Some("data")) {
        let s = ValueRef::str(
            arg0.to_yaml_string_with_options(&args_to_opts(args, kwargs, 1))
                .as_ref(),
        );
        return s.into_raw(mut_ptr_as_ref(ctx));
    }
    panic!("encode_all() missing 1 required positional argument: 'data'")
}

/// encode_all(data, sort_keys=False, ignore_private=False, ignore_none=False)
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_yaml_encode_all(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);

    if let Some(arg0) = get_call_arg(args, kwargs, 0, Some("data")) {
        let opts = args_to_opts(args, kwargs, 1);
        let results = arg0
            .as_list_ref()
            .values
            .iter()
            .map(|r| r.to_yaml_string_with_options(&opts))
            .collect::<Vec<String>>();
        let s = ValueRef::str(&results.join(YAML_STREAM_SEP));
        return s.into_raw(mut_ptr_as_ref(ctx));
    }
    panic!("encode() missing 1 required positional argument: 'data'")
}

/// decode(value)
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_yaml_decode(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(arg0) = get_call_arg(args, kwargs, 0, Some("value")) {
        match ValueRef::from_yaml(ctx, arg0.as_str().as_ref()) {
            Ok(x) => return x.into_raw(ctx),
            Err(err) => panic!("{}", err),
        }
    }
    panic!("decode() missing 1 required positional argument: 'value'")
}

/// decode_all(value)
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_yaml_decode_all(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(arg0) = get_call_arg(args, kwargs, 0, Some("value")) {
        match ValueRef::list_from_yaml_stream(ctx, arg0.as_str().as_ref()) {
            Ok(x) => return x.into_raw(ctx),
            Err(err) => panic!("{}", err),
        }
    }
    panic!("decode_all() missing 1 required positional argument: 'value'")
}

/// dump_to_file(data, sort_keys=False, ignore_private=False, ignore_none=False)
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_yaml_dump_to_file(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let data = args.arg_i(0).or(kwargs.get_by_key("data"));
    let filename = args.arg_i(1).or(kwargs.get_by_key("filename"));
    match (data, filename) {
        (Some(data), Some(filename)) => {
            let filename = filename.as_str();

            let yaml = data.to_yaml_string_with_options(&args_to_opts(args, kwargs, 2));
            std::fs::write(&filename, yaml)
                .unwrap_or_else(|e| panic!("Unable to write file '{}': {}", filename, e));
            kclvm_value_Undefined(ctx)
        }
        _ => {
            panic!("dump_to_file() missing 2 required positional arguments: 'data' and 'filename'")
        }
    }
}

/// dump_all_to_file(data, sort_keys=False, ignore_private=False, ignore_none=False)
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_yaml_dump_all_to_file(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);

    let data = args.arg_i(0).or(kwargs.get_by_key("data"));
    let filename = args.arg_i(1).or(kwargs.get_by_key("filename"));
    match (data, filename) {
        (Some(data), Some(filename)) => {
            let filename = filename.as_str();
            let opts = args_to_opts(args, kwargs, 2);
            let results = data
                .as_list_ref()
                .values
                .iter()
                .map(|r| r.to_yaml_string_with_options(&opts))
                .collect::<Vec<String>>();

            std::fs::write(filename, results.join(YAML_STREAM_SEP)).expect("Unable to write file");
            kclvm_value_Undefined(ctx)
        }
        _ => {
            panic!(
                "dump_all_to_file() missing 2 required positional arguments: 'data' and 'filename'"
            )
        }
    }
}

/// validate(value: str) -> bool
#[no_mangle]
#[runtime_fn]
pub extern "C-unwind" fn kclvm_yaml_validate(
    ctx: *mut kclvm_context_t,
    args: *const kclvm_value_ref_t,
    kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let args = ptr_as_ref(args);
    let kwargs = ptr_as_ref(kwargs);
    let ctx = mut_ptr_as_ref(ctx);

    if let Some(arg0) = get_call_arg(args, kwargs, 0, Some("value")) {
        match ValueRef::from_yaml_stream(ctx, arg0.as_str().as_ref()) {
            Ok(_) => return kclvm_value_True(ctx),
            Err(_) => return kclvm_value_False(ctx),
        }
    }
    panic!("validate() missing 1 required positional argument: 'value'")
}

fn args_to_opts(args: &ValueRef, kwargs: &ValueRef, index: usize) -> YamlEncodeOptions {
    let mut opts = YamlEncodeOptions::default();
    if let Some(sort_keys) = get_call_arg_bool(args, kwargs, index, Some("sort_keys")) {
        opts.sort_keys = sort_keys;
    }
    if let Some(ignore_private) = get_call_arg_bool(args, kwargs, index + 1, Some("ignore_private"))
    {
        opts.ignore_private = ignore_private;
    }
    if let Some(ignore_none) = get_call_arg_bool(args, kwargs, index + 2, Some("ignore_none")) {
        opts.ignore_none = ignore_none;
    }
    opts
}
