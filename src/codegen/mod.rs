use crate::ast::{Expression, Operator, Program, Statement};

pub struct CodeGen;

impl CodeGen {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, program: &Program) -> Result<String, String> {
        let mut html = String::new();
        let mut css = String::new();
        let mut js = String::new();

        for statement in &program.statements {
            match statement {
                Statement::Make(name, expr) => {
                    js.push_str(&format!("let {} = {};\n", name, self.expr_to_js(expr)));
                }
                Statement::Show(expr) => {
                    js.push_str(&format!("console.log({});\n", self.expr_to_js(expr)));
                }
                Statement::If {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    js.push_str(&format!("if ({}) {{\n", self.expr_to_js(condition)));
                    for stmt in then_branch {
                        js.push_str(&format!("  {}\n", self.stmt_to_js(stmt)));
                    }
                    if let Some(else_br) = else_branch {
                        js.push_str("} else {\n");
                        for stmt in else_br {
                            js.push_str(&format!("  {}\n", self.stmt_to_js(stmt)));
                        }
                    }
                    js.push_str("}\n");
                }
                Statement::Loop {
                    variable,
                    count,
                    body,
                } => {
                    if let Some(var) = variable {
                        if let Some(c) = count {
                            js.push_str(&format!(
                                "for (let {} = 0; {} < {}; {}++) {{\n",
                                var,
                                var,
                                self.expr_to_js(c),
                                var
                            ));
                        }
                    } else {
                        js.push_str("while (true) {\n");
                    }
                    for stmt in body {
                        js.push_str(&format!("  {}\n", self.stmt_to_js(stmt)));
                    }
                    js.push_str("}\n");
                }
                Statement::FuncDef { name, params, body } => {
                    js.push_str(&format!("function {}(", name));
                    js.push_str(&params.join(", "));
                    js.push_str(") {\n");
                    for stmt in body {
                        js.push_str(&format!("  {}\n", self.stmt_to_js(stmt)));
                    }
                    js.push_str("}\n");
                }
                Statement::Return(expr) => {
                    js.push_str(&format!("return {};\n", self.expr_to_js(expr)));
                }
                Statement::Expr(expr) => {
                    js.push_str(&format!("{};\n", self.expr_to_js(expr)));
                }
            }
        }

        html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Rex Output</title>
    <style>
{}
    </style>
</head>
<body>
    <script>
{}
    </script>
</body>
</html>"#,
            css, js
        );

        Ok(html)
    }

    fn expr_to_js(&self, expr: &Expression) -> String {
        match expr {
            Expression::Number(n) => n.to_string(),
            Expression::String(s) => format!("\"{}\"", s),
            Expression::Boolean(b) => b.to_string(),
            Expression::Null => "null".to_string(),
            Expression::Identifier(name) => name.clone(),
            Expression::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                self.expr_to_js(left),
                self.op_to_js(operator),
                self.expr_to_js(right)
            ),
            Expression::Unary { operator, operand } => {
                format!("{} {}", self.op_to_js(operator), self.expr_to_js(operand))
            }
            Expression::Call {
                function,
                arguments,
            } => {
                let args = arguments
                    .iter()
                    .map(|a| self.expr_to_js(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", function, args)
            }
            Expression::Property { object, property } => {
                format!("{}.{}", self.expr_to_js(object), property)
            }
        }
    }

    fn stmt_to_js(&self, stmt: &Statement) -> String {
        match stmt {
            Statement::Make(name, expr) => format!("let {} = {};", name, self.expr_to_js(expr)),
            Statement::Show(expr) => format!("console.log({});", self.expr_to_js(expr)),
            Statement::Return(expr) => format!("return {};", self.expr_to_js(expr)),
            Statement::Expr(expr) => format!("{};", self.expr_to_js(expr)),
            _ => String::new(),
        }
    }

    fn op_to_js(&self, op: &Operator) -> String {
        match op {
            Operator::Add => "+".to_string(),
            Operator::Subtract => "-".to_string(),
            Operator::Multiply => "*".to_string(),
            Operator::Divide => "/".to_string(),
            Operator::Modulo => "%".to_string(),
            Operator::Power => "**".to_string(),
            Operator::Equal => "===".to_string(),
            Operator::NotEqual => "!==".to_string(),
            Operator::LessThan => "<".to_string(),
            Operator::GreaterThan => ">".to_string(),
            Operator::LessOrEqual => "<=".to_string(),
            Operator::GreaterOrEqual => ">=".to_string(),
            Operator::And => "&&".to_string(),
            Operator::Or => "||".to_string(),
            Operator::Not => "!".to_string(),
        }
    }
}

impl Default for CodeGen {
    fn default() -> Self {
        Self::new()
    }
}
