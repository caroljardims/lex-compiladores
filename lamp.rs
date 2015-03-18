module lamp:
[ input :[on,off],
  output:[ligada,desligada,ring],
  t_signal:[],
  p_signal:[a,b],
  var:[],
  initially:[up(a),activate(rules)],
  on_exception:[],
  on#[a]===>[emit(ligada),up(b)],
  off#[a]===>[emit(ring),up(a)],
  on#[b]===>[emit(ring),up(b)],
  off#[b]===>[emit(desligada),up(a)]
].
