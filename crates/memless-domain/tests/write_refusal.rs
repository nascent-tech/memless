use memless_domain::refusal::WriteRefusal;

#[test]
fn a_column_count_mismatch_names_both_counts() {
    let refusal = WriteRefusal::ColumnCountMismatch { table: "users".to_string(), columns: 1, values: 2 };
    assert_eq!(refusal.to_string(), "INSERT into \"users\" names 1 columns but gives 2 values");
}

#[test]
fn a_repeated_column_names_it_and_the_table() {
    let refusal = WriteRefusal::ColumnRepeated { table: "users".to_string(), column: "id".to_string() };
    assert_eq!(refusal.to_string(), "\"id\" repeated in the write to \"users\"");
}

#[test]
fn a_disk_failure_names_the_path_and_a_closed_kind() {
    let refusal = WriteRefusal::DiskWriteFailed { path: "/tmp/x.yaml".to_string(), kind: "permission denied".to_string() };
    assert_eq!(refusal.to_string(), "cannot write file \"/tmp/x.yaml\": permission denied");
}
