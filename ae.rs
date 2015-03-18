
/* Auto estrada - Programa das Torres */

rs_prog ae_central:
[ input  : [autoriza_car, carro_entrou, outro_carro_troca,
            avise_velocidade, pista_do_carro(Z), veloc_car(Y), clima(K)],

  output : [carro_autorizado, velocidade_para_carros(X),
            troque_pista(W)],

  module carro_na_pista:
  [ input  :  [carro_entrou, autoriza_car],
    output :  [carro_autorizado, ajuste_veloc],
    t_signal: [],
    p_signal: [a, b],
    var     : [],
    initially : [up(a), activate(rules)],
    on_exception:[],
    autoriza_car#[a] ===> [emit(carro_autorizado), up(b)],
    carro_entrou#[b] ===> [emit(ajuste_veloc)]
  ],

  module controle_de_velocidade:
  [ input  :  [ajuste_veloc, clima(K), avise_velocidade, veloc_car(Y)],
    output :  [velocidade_para_carros(X)],
    t_signal: [],
    p_signal: [a],
    var     : [veloc_lim],
    initially : [veloc_lim:=60, activate(inicial), activate(controle_clima), up(a)],
    on_exception:[],
    box inicial:
    [  ajuste_veloc ===> [exit_to(controle_veloc)]
    ],
    box controle_clima:
    [ clima(K) ===> case
                 [  K=1  ---> [veloc_lim:=100, up(a)],
                    K=2  ---> [veloc_lim:=80, up(a)],
                    else ---> [veloc_lim:=60, up(a)]
                 ]
    ],
    box controle_veloc:
    [ #[a] ===> [emit(velocidade_para_carros(veloc_lim))],
      avise_velocidade ===> [emit(velocidade_para_carros(veloc_lim))],
      veloc_car(Y) ===> case
                        [ Y>veloc_lim ---> [emit(velocidade_para_carros(veloc_lim))],
                          else ---> []
                        ]
    ]
  ],

  module controle_de_pistas:
  [ input  :  [ajuste_veloc, pista_do_carro(Z), outro_carro_troca],
    output :  [troque_pista(W)],
    t_signal: [a],
    p_signal: [],
    var     : [pista],
    initially : [activate(init)],
    on_exception:[],
    box init:
    [  ajuste_veloc ===> [exit_to(controle_pistas)]
    ],
    box controle_pistas:
    [  pista_do_carro(Z) ===> [pista:=Z],
       outro_carro_troca ===> case
                        [ pista<4 ---> [pista:=pista+1,up(a)],
                          else    ---> [pista:=pista-1,up(a)]
                        ],
       #[a] ===> [emit(troque_pista(pista))]
    ]
  ]
].


/* Auto estrada - Programa das Torres */

rs_prog ae_torre:
[ input  : [carro_autorizado, requerer_autorizacao, carro_na_estrada,
            velocidade_atual(Y), solicita_velocidade, pista_atual(Z),
            outro_carro_troca_pista, velocidade_para_carros(X),
            troque_pista(W)],
  output : [autorizacao, autoriza_car, velocidade(X), carro_entrou,
            avise_velocidade, pista_do_carro(Z), veloc_car(Y),
            carro_troque_pista(W), outro_carro_troca],

  module carro_na_pista:
  [ input  :  [requerer_autorizacao, carro_autorizado, carro_na_estrada],
    output :  [carro_entrou, autorizacao, autoriza_car],
    t_signal: [],
    p_signal: [a, b, c],
    var     : [],
    initially : [up(a), activate(rules)],
    on_exception:[],
    requerer_autorizacao#[a] ===> [emit(autoriza_car), up(b)],
    carro_autorizado#[b] ===> [emit(autorizacao), up(c)],
    carro_na_estrada#[c] ===> [emit(carro_entrou)]
  ],

  module controle_de_velocidade:
  [ input  :  [velocidade_para_carros(X), solicita_velocidade, velocidade_atual(Y)],
    output :  [velocidade(X), avise_velocidade, veloc_car(Y)],
    t_signal: [],
    p_signal: [],
    var     : [],
    initially : [activate(rules)],
    on_exception:[],
    velocidade_para_carros(X) ===> [emit(velocidade(X))],
    solicita_velocidade ===> [emit(avise_velocidade)],
    velocidade_atual(Y) ===> [emit(veloc_car(Y))]
  ],

  module controle_de_pistas:
  [ input  :  [pista_atual(Z), troque_pista(W), outro_carro_troca_pista],
    output :  [pista_do_carro(Z), carro_troque_pista(W), outro_carro_troca],
    t_signal: [],
    p_signal: [],
    var     : [],
    initially : [activate(rules)],
    on_exception:[],
    pista_atual(Z) ===> [emit(pista_do_carro(Z))],
    troque_pista(W) ===> [emit(carro_troque_pista(W))],
    outro_carro_troca_pista ===> [emit(outro_carro_troca)]
  ]
].



/* Auto estrada - Programa do Carro */

rs_prog ae_carro:
[ input  : [autorizacao, velocidade(X), sensor_frente(Y),
            sensor_tras(X), carro_entrou, carro_troque_pista],
  output : [requerer_autorizacao, carro_na_estrada, velocidade_atual(X),
            solicita_velocidade, pista_atual(X),
            outro_carro_troca_pista,
            minha_pista(Q), minha_velocidade(J)],

  module inicia_carro_na_pista:
  [ input  :  [autorizacao],
    output :  [requerer_autorizacao, carro_na_estrada, recebe, carroest],
    t_signal: [],
    p_signal: [a],
    var     : [],
    initially : [emit(requerer_autorizacao), up(a), activate(rules)],
    on_exception:[],
    autorizacao#[a] ===> [emit(carro_na_estrada), emit(carroest), emit(recebe)]
  ],

  module controle_de_velocidade:
  [ input  :  [recebe, velocidade(X), sensor_frente(Y), sensor_tras(X), reduz],
    output :  [velocidade_atual(X), solicita_velocidade, ultrapassar,
               minha_velocidade(J)],
    t_signal: [print],
    p_signal: [a],
    var     : [minha_veloc, velocidade_permitida],
    initially : [activate(espera_autorizacao), activate(print_veloc)],
    on_exception:[],
    box espera_autorizacao:
    [ recebe ===> [exit_to(adequar_velocidade)]
    ],
    box adequar_velocidade:
    [ velocidade(X) ===> [minha_veloc:=X, emit(velocidade_atual(minha_veloc)),
                          activate(controle_de_distancia),
                          velocidade_permitida:=X, up(a)]
    ],
    box controle_de_distancia:
    [ sensor_frente(Y) ===> case
                    [Y<=100 & Y>=80 ---> [minha_veloc:=minha_veloc - 10,
                                          up(print)],
                     Y<80    ---> [emit(ultrapassar)],
                     Y>100   ---> [emit(solicita_velocidade)],
                     else    ---> [up(print)]
                    ],
      sensor_tras(X) ===> case
                    [X < 100 ---> [minha_veloc:=minha_veloc + 10,
                                   emit(solicita_velocidade), up(print)],
                     else    ---> [up(print)]
                    ],
      reduz    ===> [minha_veloc:=minha_veloc - 20, up(print)],
      #[print] ===> [emit(velocidade_atual(minha_veloc)), up(a)]
    ],
    box print_veloc:
    [ #[a] ===> [emit(minha_velocidade(minha_veloc))]
    ]
  ],

  module troca_de_pista:
  [ input  :  [ultrapassar, carroest, carro_entrou],
    output :  [pista_atual(X), outro_carro_troca_pista, reduz,
               minha_pista(Q)],
    t_signal: [],
    p_signal: [a],
    var     : [pista],
    initially : [activate(espera_carro_estrada), activate(print_pista)],
    on_exception:[],
    box espera_carro_estrada:
    [ carroest ===> [pista:=1, emit(pista_atual(1)),
                     exit_to(faz_ultrapassagem), up(a)]
    ],
    box faz_ultrapassagem:
    [ ultrapassar ===> case
                [pista<4 ---> [pista:=pista+1, emit(pista_atual(pista)), up(a)],
                 else    ---> [emit(outro_carro_troca_pista)]
                ],
      carro_entrou ===> case
                [pista=1 ---> [emit(reduz)],
                 else    ---> []
                ]
    ],
    box print_pista:
    [ #[a] ===> [emit(minha_pista(pista))]
    ]
  ]
].
