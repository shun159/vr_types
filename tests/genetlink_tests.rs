// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod genetlink_test {
    use vr_type::genetlink::resolve_family_id;

    #[test]
    fn resolve_family_id_test() {
        let result = resolve_family_id("TASKSTATS");
        assert_eq!(result.is_ok(), true);
    }
}

