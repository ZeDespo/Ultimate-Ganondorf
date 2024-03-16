void __thiscall
L2CFighterGanondorf::status::SpecialAirSEnd_init(L2CFighterGanondorf *this,L2CValue *return_value)

{
  L2CValue aLStack64 [16];
  L2CValue aLStack48 [16];
  
  lib::L2CValue::L2CValue(aLStack48,0xd24aef7e6);
  lib::L2CValue::L2CValue(aLStack64,0x159b5e6b33);
  FUN_7100006ef0(this,aLStack48,aLStack64);
  lib::L2CValue::~L2CValue(aLStack64);
  lib::L2CValue::~L2CValue(aLStack48);
  lib::L2CValue::L2CValue((L2CValue *)return_value,0);
  return;
}

void FUN_7100006ef0(L2CAgent *param_1,L2CValue *param_2,L2CValue *param_3)

{
  L2CValue aLStack80 [16];
  L2CValue LStack64;
  
  lib::L2CValue::L2CValue(aLStack80,0x32e468d950);
  lib::L2CAgent::clear_lua_stack(param_1);
  lib::L2CAgent::push_lua_stack(param_1,aLStack80);
  lib::L2CAgent::push_lua_stack(param_1,param_2);
  lib::L2CAgent::push_lua_stack(param_1,param_3);
  app::sv_battle_object::notify_event_msc_cmd(param_1->luaStateAgent);
  lib::L2CAgent::pop_lua_stack(param_1,1,&LStack64);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack64);
  lib::L2CValue::~L2CValue(aLStack80);
  return;
}
