/* Reconhecedor da Sequencia RST by Julius */
              7aloca
module rst:    
[ input : [r, u, t],
  output: [ok],
  t_signal: [],
  p_signal: [p1, p2], 
  var: [teste], 
  initially: [activate(rules)],
  on_exception: [],
 
  r      ===> [up(p1)],  
  u#[p1] ===> [up(p2)],
  t#[p1] ===> [],
  u#[p2] ===> [],
  r#[p2] ===> [],
  t#[p2] ===> [emit(ok)]
].
    
environment :- user_terminal.

