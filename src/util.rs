#[macro_export]
macro_rules! set_endpoint {
    { $A:ty, $B:ty, $C:literal } => {
impl crate::Endpoint for $A {
    type ResourceListKind = $B;

    const ENDPOINT: &'static str = $C;

    fn list(offset: usize, limit: usize) -> Result<Self::ResourceListKind, ::minreq::Error> {
        crate::cache::get_resource(&format!("{}/?offset={}&limit={}", Self::ENDPOINT, offset, limit))?.json::<Self::ResourceListKind>()
    }
}
};
}

#[macro_export]
macro_rules! impl_id {
    { $A:ty } => {
impl crate::Id for $A {
    fn id(&self) -> &i64 {
        &self.id
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
