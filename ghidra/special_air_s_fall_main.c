
void FUN_710000a9f0(L2CFighterCommon *param_1)

{
  byte bVar1;
  int iVar2;
  ulong uVar3;
  ulong uVar4;
  Hash40 HVar5;
  L2CValue *return_value;
  L2CValue *return_value_00;
  L2CValue *return_value_01;
  L2CValue *return_value_02;
  float fVar6;
  float fVar7;
  L2CValue LStack_90;
  L2CValue LStack_80;
  L2CValue LStack_70;
  L2CValue LStack_60;
  L2CValue LStack_50;
  
  lib::L2CValue::L2CValue(&LStack_50,0xfea97fe73);
  lib::L2CValue::L2CValue(&LStack_70,0x224ca40846);
  uVar3 = lib::L2CValue::as_integer(&LStack_50);
  uVar4 = lib::L2CValue::as_integer(&LStack_70);
  fVar6 = (float)app::lua_bind::WorkModule__get_param_float_impl
                           (param_1->moduleAccessor,uVar3,uVar4);
  lib::L2CValue::L2CValue((L2CValue *)&LStack_60,fVar6,return_value);
  lib::L2CValue::~L2CValue(&LStack_70);
  lib::L2CValue::~L2CValue(&LStack_50);
  lib::L2CValue::L2CValue((L2CValue *)&LStack_50,0.0,return_value_00);
  lib::L2CValue::operator+((L2CValue *)&LStack_60,(L2CValue *)&LStack_50,&LStack_70);
  lib::L2CValue::~L2CValue(&LStack_50);
  lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);
  fVar6 = (float)lib::L2CValue::as_number(&LStack_70);
  iVar2 = lib::L2CValue::as_integer(&LStack_50);
  app::lua_bind::WorkModule__set_float_impl(param_1->moduleAccessor,fVar6,iVar2);
  lib::L2CValue::~L2CValue(&LStack_50);
  lib::L2CValue::~L2CValue(&LStack_70);
  lib::L2CValue::L2CValue(&LStack_50,0x121f063ca1);
  lib::L2CValue::L2CValue((L2CValue *)&LStack_70,0.0,return_value_01);
  lib::L2CValue::L2CValue((L2CValue *)&LStack_80,1.0,return_value_02);
  lib::L2CValue::L2CValue(&LStack_90,false);
  HVar5 = lib::L2CValue::as_hash(&LStack_50);
  fVar6 = (float)lib::L2CValue::as_number(&LStack_70);
  fVar7 = (float)lib::L2CValue::as_number(&LStack_80);
  bVar1 = lib::L2CValue::as_bool(&LStack_90);
  app::lua_bind::MotionModule__change_motion_impl
            (param_1->moduleAccessor,HVar5,fVar6,fVar7,(bool)(bVar1 & 1),0.0,false,false);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_70);
  lib::L2CValue::~L2CValue(&LStack_50);
  FUN_7100010bc0(param_1);
  lib::L2CValue::L2CValue(&LStack_50,&DAT_71000150a0);
  lua2cpp::L2CFighterCommon::sub_shift_status_main(param_1,SUB81(&LStack_50,0));
  lib::L2CValue::~L2CValue(&LStack_50);
  lib::L2CValue::~L2CValue(&LStack_60);
  return;
}


