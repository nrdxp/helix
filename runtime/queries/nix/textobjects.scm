(apply_expression) @function.around

(comment) @comment.inside
(comment)+ @comment.around

(function_expression) @function.around

(binding
  expression: (_) @class.inside) @class.around

(function_expression
  universal: (identifier) @parameter.inside)

(formals (formal
  name: (identifier) @parameter.inside)) @parameter.around
