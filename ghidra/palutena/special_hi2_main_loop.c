/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CFighterPalutenaghidra::status::SpecialHi2_main_loop
          (L2CFighterPalutenaghidra *this,L2CValue *return_value)

{
  bool bVar1;
  byte bVar2;
  int iVar3;
  GroundCorrectKind GVar4;
  ulong uVar5;
  ulong uVar6;
  L2CValue *pLVar7;
  L2CValue *pLVar8;
  L2CValue aLStack144 [16];
  L2CValue aLStack128 [16];
  L2CValue aLStack112 [16];
  L2CValue LStack96;
  L2CValue LStack80;
  
  lua2cpp::L2CFighterCommon::sub_transition_group_check_air_cliff(this,&LStack80);
  bVar1 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack80);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
  if ((bVar1 & 1U) != 0) {
    iVar3 = 1;
    goto LAB_71000166a4;
  }
  lib::L2CValue::L2CValue((L2CValue *)&LStack96,_FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
  iVar3 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
  iVar3 = app::lua_bind::WorkModule__get_int_impl(this->moduleAccessor,iVar3);
  lib::L2CValue::L2CValue((L2CValue *)&LStack80,iVar3);
  lib::L2CValue::L2CValue(aLStack128,0x1086bc4a93);
  lib::L2CValue::L2CValue(aLStack144,0x14693eb7b7);
  uVar5 = lib::L2CValue::as_integer(aLStack128);
  uVar6 = lib::L2CValue::as_integer(aLStack144);
  iVar3 = app::lua_bind::WorkModule__get_param_int_impl(this->moduleAccessor,uVar5,uVar6);
  lib::L2CValue::L2CValue(aLStack112,iVar3);
  uVar5 = lib::L2CValue::operator<=(aLStack112,(L2CValue *)&LStack80);
  lib::L2CValue::~L2CValue(aLStack112);
  lib::L2CValue::~L2CValue(aLStack144);
  lib::L2CValue::~L2CValue(aLStack128);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
  if ((uVar5 & 1) == 0) {
    bVar2 = app::lua_bind::StatusModule__is_changing_impl(this->moduleAccessor);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,(bool)(bVar2 & 1));
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
    uVar5 = lib::L2CValue::operator==((L2CValue *)&LStack96,(L2CValue *)&LStack80);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
    if ((uVar5 & 1) != 0) {
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
LAB_7100016558:
      pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&this->globalTable,0x16);
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
      uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
      if ((uVar5 & 1) == 0) {
        lib::L2CValue::L2CValue((L2CValue *)&LStack80,GROUND_CORRECT_KIND_AIR);
        GVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack80);
        app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar4);
      }
      else {
        lib::L2CValue::L2CValue((L2CValue *)&LStack80,GROUND_CORRECT_KIND_GROUND);
        GVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack80);
        app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar4);
      }
      goto LAB_7100016694;
    }
    pLVar8 = &this->globalTable;
    pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x17);
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
    if ((uVar5 & 1) != 0) {
      pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x16);
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,SITUATION_KIND_AIR);
      uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
      if ((uVar5 & 1) != 0) {
        lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
        goto LAB_7100016558;
      }
    }
    pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x17);
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
    if ((uVar5 & 1) != 0) {
      pLVar8 = &LStack96;
      goto LAB_7100016698;
    }
    pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x16);
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    if ((uVar5 & 1) != 0) goto LAB_7100016558;
  }
  else {
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,false);
    lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0xb0,(L2CValue)0xa0);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
LAB_7100016694:
    pLVar8 = &LStack80;
LAB_7100016698:
    lib::L2CValue::~L2CValue((L2CValue *)pLVar8);
  }
  iVar3 = 0;
LAB_71000166a4:
  lib::L2CValue::L2CValue((L2CValue *)return_value,iVar3);
  return;
}
