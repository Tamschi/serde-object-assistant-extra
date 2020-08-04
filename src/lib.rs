use {linkme::distributed_slice, std::borrow::Cow};

pub enum VariantKind {
    Unit,
    Newtype,
    Tuple(usize),
    Struct(Cow<'static, [Cow<'static, str>]>),
}

/// Called between EnumAccess::variant_seed and one of VariantAccess's methods.
#[distributed_slice]
pub static ENUM_VARIANT_ASSISTS: [fn() -> Option<VariantKind>] = [..];

pub fn enum_variant_hint() -> Option<VariantKind> {
    // If it were possible to get type-GUIDs for non-'static types, this would be doable a lot more nicely.
    // Without that, we can't inspect the EnumAccess or VariantAccess involved safely, though.
    // Unfortunately, the RFC for non-'static TypeIds was rejected (<https://github.com/rust-lang/rust/issues/41875>),
    // so this is the best I can do.

    for enum_variant_assist in ENUM_VARIANT_ASSISTS.iter() {
        if let result @ Some(_) = enum_variant_assist() {
            return result;
        }
    }
    None
}
