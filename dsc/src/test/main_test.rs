#[path = "../main/main.rs"]
mod main;

#[cfg(test)]
#[test]
fn test_column_to_sql(){
    let column_id = main::Column {
        name: "id".to_string(),
        data_type: "integer".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    let column_name =main::Column {
        name: "name".to_string(),
        data_type: "varchar(255)".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    let column_age = main::Column {
        name: "age".to_string(),
        data_type: "integer".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    let column_created_at = main::Column {
        name: "created_at".to_string(),
        data_type: "timestamp".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    assert_eq!(column_id.to_sql(), "id integer,\n");
    assert_eq!(column_name.to_sql(), "name varchar(255),\n");
    assert_eq!(column_age.to_sql(), "age integer,\n");
    assert_eq!(column_created_at.to_sql(), "created_at timestamp,\n");
}
