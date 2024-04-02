/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CFighterPalutenaghidra::status::SpecialHi_main_loop
          (L2CFighterPalutenaghidra *this,L2CValue *return_value)

{
  bool bVar1;
  byte bVar2;
  GroundCorrectKind GVar3;
  int iVar4;
  ulong uVar5;
  L2CValue *pLVar6;
  Hash40 HVar7;
  L2CValue *pLVar8;
  L2CValue *return_value_00;
  L2CValue *return_value_01;
  L2CValue *return_value_02;
  L2CValue *return_value_03;
  float fVar9;
  float fVar10;
  L2CValue aLStack144 [16];
  L2CValue aLStack128 [16];
  L2CValue LStack112;
  L2CValue LStack96;
  
  lua2cpp::L2CFighterCommon::sub_transition_group_check_air_cliff(this,&LStack96);
  bVar1 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack96);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
  if ((bVar1 & 1U) != 0) {
    iVar4 = 1;
    goto LAB_7100017a44;
  }
  bVar2 = app::lua_bind::StatusModule__is_changing_impl(this->moduleAccessor);
  lib::L2CValue::L2CValue((L2CValue *)&LStack112,(bool)(bVar2 & 1));
  lib::L2CValue::L2CValue((L2CValue *)&LStack96,true);
  uVar5 = lib::L2CValue::operator==((L2CValue *)&LStack112,(L2CValue *)&LStack96);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
  if ((uVar5 & 1) == 0) {
    pLVar8 = &this->globalTable;
    pLVar6 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x17);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    if ((uVar5 & 1) != 0) {
      pLVar6 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x16);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,SITUATION_KIND_AIR);
      uVar5 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      if ((uVar5 & 1) != 0) {
        lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
        goto LAB_7100017528;
      }
    }
    pLVar6 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x17);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    if ((uVar5 & 1) != 0) {
      pLVar8 = &LStack112;
      goto LAB_71000179c8;
    }
    pLVar6 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x16);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
    if ((uVar5 & 1) != 0) goto LAB_7100017528;
  }
  else {
    lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
LAB_7100017528:
    pLVar6 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&this->globalTable,0x16);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar6,(L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    if ((uVar5 & 1) == 0) {
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,GROUND_CORRECT_KIND_AIR);
      GVar3 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
      app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar3);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,_FIGHTER_KINETIC_TYPE_PALUTENA_SPECIAL_HI_AIR);
      iVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
      app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar4);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::L2CValue
                ((L2CValue *)&LStack112,_FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
      iVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack112);
      bVar2 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar4);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,(bool)(bVar2 & 1));
      bVar1 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
      if ((bVar1 & 1U) == 0) {
        lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x14dd899136);
        lib::L2CValue::L2CValue((L2CValue *)&LStack112,0.0,return_value_02);
        lib::L2CValue::L2CValue(aLStack128,1.0,return_value_03);
        lib::L2CValue::L2CValue(aLStack144,false);
        HVar7 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
        fVar9 = (float)lib::L2CValue::as_number((L2CValue *)&LStack112);
        fVar10 = (float)lib::L2CValue::as_number(aLStack128);
        bVar2 = lib::L2CValue::as_bool(aLStack144);
        app::lua_bind::MotionModule__change_motion_impl
                  (this->moduleAccessor,HVar7,fVar9,fVar10,(bool)(bVar2 & 1),0.0,false,false);
        lib::L2CValue::~L2CValue(aLStack144);
        lib::L2CValue::~L2CValue(aLStack128);
        lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
        lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
        lib::L2CValue::L2CValue
                  ((L2CValue *)&LStack96,_FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        iVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
        app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar4);
      }
      else {
        lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x14dd899136);
        HVar7 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
        app::lua_bind::MotionModule__change_motion_inherit_frame_impl
                  (this->moduleAccessor,HVar7,-1.0,1.0,0.0,false,false);
      }
    }
    else {
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,GROUND_CORRECT_KIND_GROUND);
      GVar3 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
      app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar3);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,_FIGHTER_KINETIC_TYPE_GROUND_STOP);
      iVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
      app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar4);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::L2CValue
                ((L2CValue *)&LStack112,_FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
      iVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack112);
      bVar2 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar4);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,(bool)(bVar2 & 1));
      bVar1 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
      if ((bVar1 & 1U) == 0) {
        lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x105c3c1e76);
        lib::L2CValue::L2CValue((L2CValue *)&LStack112,0.0,return_value_00);
        lib::L2CValue::L2CValue(aLStack128,1.0,return_value_01);
        lib::L2CValue::L2CValue(aLStack144,false);
        HVar7 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
        fVar9 = (float)lib::L2CValue::as_number((L2CValue *)&LStack112);
        fVar10 = (float)lib::L2CValue::as_number(aLStack128);
        bVar2 = lib::L2CValue::as_bool(aLStack144);
        app::lua_bind::MotionModule__change_motion_impl
                  (this->moduleAccessor,HVar7,fVar9,fVar10,(bool)(bVar2 & 1),0.0,false,false);
        lib::L2CValue::~L2CValue(aLStack144);
        lib::L2CValue::~L2CValue(aLStack128);
        lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
        lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
        lib::L2CValue::L2CValue
                  ((L2CValue *)&LStack96,_FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        iVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
        app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar4);
      }
      else {
        lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x105c3c1e76);
        HVar7 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
        app::lua_bind::MotionModule__change_motion_inherit_frame_impl
                  (this->moduleAccessor,HVar7,-1.0,1.0,0.0,false,false);
      }
    }
    pLVar8 = &LStack96;
LAB_71000179c8:
    lib::L2CValue::~L2CValue((L2CValue *)pLVar8);
  }
  bVar2 = app::lua_bind::MotionModule__is_end_impl(this->moduleAccessor);
  lib::L2CValue::L2CValue((L2CValue *)&LStack96,(bool)(bVar2 & 1));
  bVar1 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack96);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
  if ((bVar1 & 1U) != 0) {
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,_FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2);
    lib::L2CValue::L2CValue((L2CValue *)&LStack112,false);
    lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0xa0,(L2CValue)0x90);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack112);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
  }
  iVar4 = 0;
LAB_7100017a44:
  lib::L2CValue::L2CValue((L2CValue *)return_value,iVar4);
  return;
}
