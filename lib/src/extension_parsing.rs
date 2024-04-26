pub(crate) mod skip_empty {
    use chrono::{DateTime, Utc};

    #[inline(always)]
    pub(crate) fn i64(v: &i64) -> bool {
        *v == 0
    }

    #[inline(always)]
    pub(crate) fn bool(v: &bool) -> bool {
        *v == false
    }

    #[inline(always)]
    pub(crate) fn date(v: &DateTime<Utc>) -> bool {
        *v == DateTime::<Utc>::default()
    }
}
