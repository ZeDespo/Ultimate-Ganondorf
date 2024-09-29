
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_71000150a0(void *param_1)

{
  bool bVar1;
  L2CValue *this;
  ulong uVar2;
  L2CValue *in_x8;
  L2CValue LStack_50;
  L2CValue LStack_40;
  
  this = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)((long)param_1 + 200),0x16);
  lib::L2CValue::L2CValue(&LStack_40,_SITUATION_KIND_GROUND);
  uVar2 = lib::L2CValue::operator==(this,(L2CValue *)&LStack_40);
  lib::L2CValue::~L2CValue(&LStack_40);
  bVar1 = (uVar2 & 1) != 0;
  if (bVar1) {
    lib::L2CValue::L2CValue(&LStack_40,_FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END);
    lib::L2CValue::L2CValue(&LStack_50,false);
    lua2cpp::L2CFighterBase::change_status(param_1,SUB81(&LStack_40,0),SUB81(&LStack_50,0));
    lib::L2CValue::~L2CValue(&LStack_50);
    lib::L2CValue::~L2CValue(&LStack_40);
  }
  lib::L2CValue::L2CValue(in_x8,(uint)bVar1);
  return;
}
