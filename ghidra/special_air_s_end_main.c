
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CFighterGanon::status::SpecialAirSEnd_main(L2CFighterGanon *this,L2CValue *return_value)


/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_7100010b20(void *param_1)

{
  GroundCorrectKind GVar1;
  L2CValue LStack_40;
  L2CValue LStack_30;
  
  lib::L2CValue::L2CValue(&LStack_30,_SITUATION_KIND_GROUND);
  lua2cpp::L2CFighterBase::set_situation(param_1,SUB81(&LStack_30,0));
  lib::L2CValue::L2CValue(&LStack_40,GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
  GVar1 = lib::L2CValue::as_integer(&LStack_40);
  app::lua_bind::GroundModule__correct_impl
            (*(BattleObjectModuleAccessor **)((long)param_1 + 0x40),GVar1);
  return;
}



{
  byte bVar1;
  int iVar2;
  Hash40 HVar3;
  float fVar4;
  float fVar5;
  L2CValue aLStack128 [16];
  L2CValue aLStack112 [16];
  L2CValue aLStack96 [16];
  L2CValue aLStack80 [16];
  
  lib::L2CValue::L2CValue(aLStack80,0xd2b3a620b);
  lib::L2CValue::L2CValue(aLStack96,0.0);
  lib::L2CValue::L2CValue(aLStack112,1.0);
  lib::L2CValue::L2CValue(aLStack128,false);
  HVar3 = lib::L2CValue::as_hash(aLStack80);
  fVar4 = (float)lib::L2CValue::as_number(aLStack96);
  fVar5 = (float)lib::L2CValue::as_number(aLStack112);
  bVar1 = lib::L2CValue::as_bool(aLStack128);
  app::lua_bind::MotionModule__change_motion_impl
            (this->moduleAccessor,HVar3,fVar4,fVar5,(bool)(bVar1 & 1),0.0,false,false);
  FUN_7100010b20(this);
  lib::L2CValue::L2CValue(aLStack80,_FIGHTER_KINETIC_TYPE_GROUND_STOP);
  iVar2 = lib::L2CValue::as_integer(aLStack80);
  app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar2);
  lib::L2CValue::L2CValue(aLStack80,SpecialAirSEnd_main_loop);
  lua2cpp::L2CFighterCommon::sub_shift_status_main(this,(L2CValue)0xb0);
  return;
}
