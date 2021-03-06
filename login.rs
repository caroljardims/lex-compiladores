rs_prog login:
[ input : [name(X), password(Y)],
  output: [loginSuccessful, loginUnsuccessful],

  module get_login:
  [ input   : [name(X), password(Y), goodLogin, badLogin, tooManyAttempts],
    output  : [loginSuccessful, loginUnsuccessful, checkLogin(X,Y)],
    t_signal:[],
    p_signal: [gotName(X), gotPassword(Y)],
    var   : [],
    initially: [nl, write('Enter name and password'), activate(rules)],
    on_exception:[],

    name(X) ===> [up(gotName(X))],
    password(X) ===> [up(gotPassword(X))],
    #[gotName(X), gotPassword(Y)] ===> [emit(checkLogin(X,Y))],
    goodLogin ===> [emit(loginSuccessful)],
    badLogin  ===> [nl, write('Please, repeat name and password')],
    tooManyAttempts ===> [emit(loginUnsuccessful)]
  ],

  module check_login:
  [ input   : [checkLogin(X,Y)],
    output  : [goodLogin, badLogin, tooManyAttempts],
    t_signal:[],
    p_signal: [],
    var     : [count],
    initially: [count := 0, activate(rules)],
    on_exception:[],
 ~
    checkLogin(X,Y) ===> case
                        [ user(X,Y) ---> [count:=0, emit(goodLogin)],
                          count>=2  ---> [emit(tooManyAttempts)],
                          else      ---> [count:=count+1, emit(badLogin)]  ]
  ]
].

user(sst, aaa).
user(lvt, bbb).
