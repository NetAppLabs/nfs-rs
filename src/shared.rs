/// Struct describing an NFS timestamp.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Time {
    pub seconds: u32,
    pub nseconds: u32,
}
