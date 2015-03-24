/* Podos System by Julius */

rs_prog podos:
[ 
  input : [ 
            start_button,        /* inicia o processo do podos */
            stop_button,         /* termina o processo do podos */
            heart_button,        /* mostra a batidas do coracao por segundo */
            veloc_button,        /* mostra a velocidade atual */
            distance_button,     /* mostra a distancia percorrida */
            acel(A),             /* aceleracoes resultante */
            pulso,               /* sinal de batimento cardiaco */
            sec                  /* segundo */
          ],

  output: [
            display_heart(X),       /* mostra as batidas do coracao */
            display_veloc(X),       /* mostra a velocidade atual */
            display_distance(X)     /* mostra distancia percorrida */
          ],


module keyboard_module:
[ input : [ 
            start_button,        
            stop_button,        
            heart_button,  
            veloc_button,     
            distance_button
          ],


  output: [
            cmd_heart,   
            cmd_veloc,    
            cmd_distance,
            start_system,
            stop_system     
          ],

  t_signal: [],
  p_signal: [],
  var: [],

  initially: [  
               activate(rules)
             ],
  
  on_exception: [],

  start_button ===> [emit(start_system)],        
  stop_button ===> [emit(stop_system)],  
  heart_button ===> [emit(cmd_heart)],  
  veloc_button ===> [emit(cmd_veloc)],    
  distance_button ===> [emit(cmd_distance)]   

],

 
module control_module:
[ input : [ 
            start_system,
            stop_system,
            cmd_heart,
            cmd_veloc,
            cmd_distance   
          ],


  output: [
            start_pulsacao,
            start_integrator,
            stop_pulsacao,
            stop_integrator,
            cmd_pulsacao,
            cmd_integrator,
            cmd_velocity      
          ],

  t_signal: [],
  p_signal: [],
  var: [],

  initially: [activate(rules)],   
  
  on_exception: [],


    start_system ===> [emit(start_pulsacao), emit(start_integrator)],
    stop_system ===> [emit(stop_pulsacao), emit(stop_integrator)],     
    cmd_heart ===> [emit(cmd_pulsacao)],
    cmd_distance ===> [emit(cmd_integrator)],
    cmd_veloc ===> [emit(cmd_velocity)]

 ],


module pulsacao_module:
[ input : [ 
            start_pulsacao,
            stop_pulsacao,
            pulso,
            sec,
            cmd_pulsacao   
          ],


  output: [
            display_heart(X)      
          ],

  t_signal: [],
  p_signal: [],
  var: [bat_sec, batimentos],

  initially: [activate(box_initial)],   
  
  on_exception: [],


  box box_initial:
  [
    start_pulsacao ===> [activate(box_operation), bat_sec:=0, batimentos:=0],
    cmd_pulsacao ===> [emit(display_heart(bat_sec))]
  ],

  box box_operation:
  [
    sec ===> [bat_sec:=batimentos, batimentos:=0],
    pulso ===> [batimentos:=batimentos+1],

    stop_pulsacao ===> [deactivate]    
  ]

 ],

  
module integrator_module:
[ input : [ 
            start_integrator,
            stop_integrator,
            acel(A),
            cmd_integrator,   
            cmd_velocity
          ],


  output: [
            display_veloc(X),       
            display_distance(X)           
          ],

  t_signal: [],
  p_signal: [p1, p2, p3, p4],
  var: [acel_result, acel_ant, vel_result, vel_ant, dist_result, dist_ant, dist],

  initially: [activate(box_initial)],   
  
  on_exception: [],


  box box_initial:
  [
    start_integrator ===> [activate(box_integrator), acel_ant:=0, vel_ant:=0, dist_ant:=0],
    cmd_integrator ===> [emit(display_distance(dist_result))],
    cmd_velocity ===> [emit(display_veloc(vel_result))]      
  ],

  box box_integrator:
  [

    acel(A) ===> [acel_result:=A, up(p1)],
    #[p1] ===> [vel_result:=(acel_ant+acel_result)/2, up(p2)],
    #[p2] ===> [dist:=(vel_result+vel_ant)/2, up(p3)],
    #[p3] ===> [dist_result:=dist+dist_ant, up(p4)],
    #[p4] ===> [dist_ant:=dist_result, vel_ant:=vel_result, acel_ant:=acel_result],

    stop_integrator ===> [deactivate]    
  ]

 ]


].

environment :- user_terminal.

