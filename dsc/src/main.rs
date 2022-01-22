struct Column {
    name: String,
    data_type: String,
    comment: String,
    default: String,
}
struct Constraint {
    name: String,
    columns: Vec<Column>,
}
struct UniqueConstraint {
    constraint: Constraint,
}
struct PrimaryKey {
    unique_constraint: UniqueConstraint,
}
struct ForeignKey {
    constraint: Constraint,
    referenced_table: Table,
    referenced_columns: Vec<Column>,
}
struct Table {
    name: String,
    columns: Vec<Column>,
   // primary_key: PrimaryKey,
    constraints: Vec<Constraint>,
    comment: String,
}
impl Table {
    fn new(name: String, columns: Vec<Column>, constraints: Vec<Constraint>, comment: String) -> Self {
        Table {
            name,
            columns,
           // primary_key,
            constraints,
            comment,
        }
    }
    fn to_sql(&self) -> String {
        let mut sql = String::new();
        sql.push_str(&format!("CREATE TABLE {} (\n", self.name));
        for column in &self.columns {
            sql.push_str(&format!("    {} {}", column.name, column.data_type));
            if !column.default.is_empty() {
                sql.push_str(&format!(" DEFAULT {}", column.default));
            }
            if !column.comment.is_empty() {
                sql.push_str(&format!(" COMMENT '{}'", column.comment));
            }
            sql.push_str(",\n");
        }
        sql.pop();
        sql.pop();
        sql.push_str("\n)");
        return sql;
    }
}
    

fn main() {
    let mut columns: Vec<Column> = Vec::new();
    let column_id = Column {
        name: "id".to_string(),
        data_type: "integer".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    let column_name = Column {
        name: "name".to_string(),
        data_type: "varchar(255)".to_string(),
        comment: "".to_string(),
        default: "".to_string(),
    };
    columns.push(column_id);
    columns.push(column_name);
    // let primary_key = PrimaryKey {
    //     unique_constraint: UniqueConstraint {
    //         constraint: Constraint {
    //             name: "TEST_PK".to_string(),
    //             columns: vec![column_id.clone()]
    //         },
    //     },
    // };
    let table = Table::new(
        "TEST".to_string(),
        columns,
       // primary_key,
        Vec::new(),
        "".to_string(),
    );
    print!("{}", table.to_sql());
}
