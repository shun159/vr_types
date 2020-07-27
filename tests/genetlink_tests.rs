// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod genetlink_test {
    use vr_type::genetlink::resolve_family_id;

    #[test]
    fn resolve_family_id_test() {
        let expect = 29;
        let response = resolve_family_id("TASKSTATS").unwrap();
        assert_eq!(expect, response);
    }
}
