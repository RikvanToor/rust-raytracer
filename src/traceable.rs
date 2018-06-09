use ray::Ray;

pub struct Traceable<'a> {
    pub ray: &'a Ray,
    pub output: usize,
    pub trace_type: TraceType,
    pub depth: u8,
}

pub enum TraceType {
    PrimaryRay,
    ShadowRay,
}
