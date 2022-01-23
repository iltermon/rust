
pub trait TableTrait {
    fn to_sql(&self) -> String;
    fn new(
        name: String,
        columns: Vec<Column>,
        constraints: Vec<Constraint>,
        comment: String,
    ) -> Table {
        return Table {
            name,
            columns,
            // primary_key,
            constraints,
            comment,
        };
    }
}
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub comment: String,
    pub default: String,
}

impl Column {
    fn new_from(column: &Column) -> Column {
        Column {
            name: column.name.clone(),
            data_type: column.data_type.clone(),
            comment: column.comment.clone(),
            default: column.default.clone(),
        }
    }
    pub fn to_sql(&self) -> String {
        let mut sql = String::new();
        sql.push_str(&self.name);
        sql.push_str(" ");
        sql.push_str(&self.data_type);
        if !self.default.is_empty() {
            sql.push_str(" DEFAULT ");
            sql.push_str(&self.default);
        }
        // if self.comment.len() > 0 {
        //     sql.push_str(" COMMENT ");
        //     sql.push_str(&self.comment);
        // }
        sql.push_str(",\n");
        return sql;
    }
}
pub struct Constraint {
    name: String,
    columns: Vec<Column>,
}
pub struct UniqueConstraint {
    constraint: Constraint,
}
pub struct PrimaryKey {
    unique_constraint: UniqueConstraint,
}
pub struct ForeignKey {
    constraint: Constraint,
    referenced_table: Table,
    referenced_columns: Vec<Column>,
}
pub struct Table {
    name: String,
    columns: Vec<Column>,
    // primary_key: PrimaryKey,
    constraints: Vec<Constraint>,
    comment: String,
}
impl TableTrait for Table {
    fn new(
        name: String,
        columns: Vec<Column>,
        constraints: Vec<Constraint>,
        comment: String,
    ) -> Table {
        return Table {
            name,
            columns,
            // primary_key,
            constraints,
            comment,
        };
    }
    fn to_sql(&self) -> String {
        let mut sql = String::new();
        sql.push_str(&format!("CREATE TABLE {} (\n", self.name));
        for column in &self.columns {
            sql.push_str("\t");
            sql.push_str(&column.to_sql());
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
    columns.push(Column::new_from(&column_id));
    columns.push(column_name);
    let table = Table::new(
        "TEST".to_string(),
        columns,
        // primary_key,
        Vec::new(),
        "".to_string(),
    );
    print!("{}", table.to_sql());
}
