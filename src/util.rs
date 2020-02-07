/// Gets the location of an API resource from a full url, minus the url
/// and common prefix, e.g. "https://pokeapi.co/api/v2/"

#[macro_export]
macro_rules! set_endpoint {
    { $A:ty, $B:tt, $C:literal } => {
impl crate::Endpoint for $A {
    type ResourceListKind = $B<$A>;

    const ENDPOINT: &'static str = $C;

    fn list(offset: usize, limit: usize) -> Result<Self::ResourceListKind, ::minreq::Error> {
        crate::cache::get_resource(&format!("{}/?offset={}&limit={}", Self::ENDPOINT, offset, limit))?.json::<Self::ResourceListKind>()
    }

    fn full_list() -> Result<Self::ResourceListKind, ::minreq::Error> {
        crate::cache::get_resource(&format!("{}/?offset=0&limit=-1", Self::ENDPOINT))?.json::<Self::ResourceListKind>()
    }
}
};
}

#[macro_export]
macro_rules! impl_id {
    { $A:ty } => {
impl crate::Id for $A {
    fn id(&self) -> i64 {
        self.id
    }
}
};
}

#[macro_export]
macro_rules! impl_named {
    { $A:ty } => {
impl crate::Named for $A {
    fn name(&self) -> &String {
        &self.name
    }
}
};
}

#[macro_export]
macro_rules! impl_id_and_named {
    { $A:ty } => {
crate::impl_named!{$A}
crate::impl_id!{$A}
};
}
