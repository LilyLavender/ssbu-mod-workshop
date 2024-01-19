
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CFighterMetaknight::status::SpecialNSpin_main(L2CFighterMetaknight *this,L2CValue *return_value )

{
  byte bVar1;
  int iVar2;
  int iVar3;
  ulong uVar4;
  ulong uVar5;
  Hash40 HVar6;
  L2CValue *this_00;
  L2CValue *this_01;
  L2CValue *return_value_00;
  L2CValue *return_value_01;
  L2CValue *return_value_02;
  L2CValue *return_value_03;
  L2CValue *return_value_04;
  L2CValue *return_value_05;
  L2CValue *return_value_06;
  L2CValue *return_value_07;
  L2CValue *return_value_08;
  L2CValue *return_value_09;
  L2CValue *return_value_10;
  L2CValue *return_value_11;
  L2CValue *return_value_12;
  L2CValue *return_value_13;
  float fVar7;
  float fVar8;
  uint uVar9;
  long lVar10;
  L2CValue aLStack240 [16];
  L2CValue aLStack224 [16];
  L2CValue LStack208;
  L2CValue LStack192;
  L2CValue LStack176;
  L2CValue aLStack160 [16];
  L2CValue aLStack144 [16];
  L2CValue aLStack128 [16];
  L2CValue aLStack112 [16];
  ulong local_60;
  ulong uStack88;
  
  lib::L2CValue::L2CValue(aLStack112,0xf899192aa);
  lib::L2CValue::L2CValue(aLStack128,0x13a970224e);
  uVar4 = lib::L2CValue::as_integer(aLStack112);
  uVar5 = lib::L2CValue::as_integer(aLStack128);
  iVar2 = app::lua_bind::WorkModule__get_param_int_impl(this->moduleAccessor,uVar4,uVar5);
  lib::L2CValue::L2CValue((L2CValue *)&local_60,iVar2,return_value_00);
  lib::L2CValue::L2CValue
            (aLStack144,_FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_UNABLE_COUNTER,
             (L2CValue *)&LUA_SCRIPT_LINE_MAX);
  iVar2 = lib::L2CValue::as_integer((L2CValue *)&local_60);
  iVar3 = lib::L2CValue::as_integer(aLStack144);
  app::lua_bind::WorkModule__set_int_impl(this->moduleAccessor,iVar2,iVar3);
  lib::L2CValue::~L2CValue(aLStack144);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  lib::L2CValue::~L2CValue(aLStack128);
  lib::L2CValue::~L2CValue(aLStack112);
  lib::L2CValue::L2CValue((L2CValue *)&local_60,0xe0b4623a8);
  lib::L2CValue::L2CValue(aLStack112,0.0,return_value_01);
  lib::L2CValue::L2CValue(aLStack128,1.0,return_value_02);
  lib::L2CValue::L2CValue(aLStack144,false);
  HVar6 = lib::L2CValue::as_hash((L2CValue *)&local_60);
  fVar7 = (float)lib::L2CValue::as_number(aLStack112);
  fVar8 = (float)lib::L2CValue::as_number(aLStack128);
  bVar1 = lib::L2CValue::as_bool(aLStack144);
  app::lua_bind::MotionModule__change_motion_impl
            (this->moduleAccessor,HVar6,fVar7,fVar8,(bool)(bVar1 & 1),0.0,false,false);
  lib::L2CValue::~L2CValue(aLStack144);
  lib::L2CValue::~L2CValue(aLStack128);
  lib::L2CValue::~L2CValue(aLStack112);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  lib::L2CValue::L2CValue(aLStack112,0xf899192aa);
  lib::L2CValue::L2CValue(aLStack128,0xfd5a471dd);
  uVar4 = lib::L2CValue::as_integer(aLStack112);
  uVar5 = lib::L2CValue::as_integer(aLStack128);
  fVar7 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar4,uVar5 );
  lib::L2CValue::L2CValue((L2CValue *)&local_60,fVar7,return_value_03);
  fVar7 = (float)lib::L2CValue::as_number((L2CValue *)&local_60);
  app::lua_bind::MotionModule__set_rate_impl(this->moduleAccessor,fVar7);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  lib::L2CValue::~L2CValue(aLStack128);
  lib::L2CValue::~L2CValue(aLStack112);
  bVar1 = app::lua_bind::StopModule__is_stop_impl(this->moduleAccessor);
  lib::L2CValue::L2CValue(aLStack112,(bool)(bVar1 & 1));
  lib::L2CValue::L2CValue((L2CValue *)&local_60,false);
  uVar4 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&local_60);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  lib::L2CValue::~L2CValue(aLStack112);
  if ((uVar4 & 1) != 0) {
    lib::L2CValue::L2CValue(aLStack128,false);
    FUN_710001ca30(aLStack112,this,aLStack128);
    lib::L2CValue::~L2CValue(aLStack112);
    lib::L2CValue::~L2CValue(aLStack128);
  }
  this_00 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&this->globalTable,0x15);
  lib::L2CValue::L2CValue((L2CValue *)&local_60,&DAT_710001d7f0);
  lib::L2CValue::operator=(this_00,(L2CValue *)&local_60);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  FUN_710001d850(this);
  lib::L2CValue::L2CValue((L2CValue *)&local_60,0xf899192aa);
  lib::L2CValue::L2CValue(aLStack160,0xf2d0c3c2e);
  uVar4 = lib::L2CValue::as_integer((L2CValue *)&local_60);
  uVar5 = lib::L2CValue::as_integer(aLStack160);
  fVar7 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar4,uVar5 );
  lib::L2CValue::L2CValue(aLStack144,fVar7,return_value_04);
  lib::L2CValue::~L2CValue(aLStack160);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  lib::L2CValue::L2CValue((L2CValue *)&local_60,0xf899192aa);
  lib::L2CValue::L2CValue((L2CValue *)&LStack176,0x11ba5b8b39);
  uVar4 = lib::L2CValue::as_integer((L2CValue *)&local_60);
  uVar5 = lib::L2CValue::as_integer((L2CValue *)&LStack176);
  fVar7 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar4,uVar5 );
  lib::L2CValue::L2CValue(aLStack160,fVar7,return_value_05);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack176);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  fVar7 = (float)app::lua_bind::ControlModule__get_stick_x_impl(this->moduleAccessor);
  lib::L2CValue::L2CValue((L2CValue *)&local_60,fVar7,return_value_06);
  uVar4 = lib::L2CValue::operator<=(aLStack144,(L2CValue *)&local_60);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  if ((uVar4 & 1) == 0) {
    fVar7 = (float)app::lua_bind::ControlModule__get_stick_x_impl(this->moduleAccessor);
    lib::L2CValue::L2CValue((L2CValue *)&local_60,fVar7,return_value_10);
    lib::L2CValue::operator-(aLStack144,&LStack176);
    uVar4 = lib::L2CValue::operator<=((L2CValue *)&local_60,(L2CValue *)&LStack176);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack176);
    lib::L2CValue::~L2CValue((L2CValue *)&local_60);
    if ((uVar4 & 1) == 0) goto LAB_7100006b40;
    lib::L2CValue::operator-(aLStack160,&LStack192);
    fVar7 = (float)app::lua_bind::PostureModule__lr_impl(this->moduleAccessor);
    lib::L2CValue::L2CValue((L2CValue *)&LStack208,fVar7,return_value_11);
    lib::L2CValue::operator*((L2CValue *)&LStack192,(L2CValue *)&LStack208,&LStack176);
    lib::L2CValue::L2CValue(aLStack224,0.0,return_value_12);
    lib::L2CValue::L2CValue(aLStack240,0.0,return_value_13);
    uVar4 = lib::L2CValue::as_number((L2CValue *)&LStack176);
    lVar10 = lib::L2CValue::as_number(aLStack224);
    uVar9 = lib::L2CValue::as_number(aLStack240);
    local_60 = uVar4 & 0xffffffff | lVar10 << 0x20;
    uStack88 = (ulong)uVar9;
    app::lua_bind::KineticModule__add_speed_impl(this->moduleAccessor,(Vector3f *)&local_60);
    lib::L2CValue::~L2CValue(aLStack240);
    lib::L2CValue::~L2CValue(aLStack224);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack176);
    this_01 = &LStack208;
  }
  else {
    fVar7 = (float)app::lua_bind::PostureModule__lr_impl(this->moduleAccessor);
    lib::L2CValue::L2CValue((L2CValue *)&LStack192,fVar7,return_value_07);
    lib::L2CValue::operator*(aLStack160,(L2CValue *)&LStack192,&LStack176);
    lib::L2CValue::L2CValue((L2CValue *)&LStack208,0.0,return_value_08);
    lib::L2CValue::L2CValue(aLStack224,0.0,return_value_09);
    uVar4 = lib::L2CValue::as_number((L2CValue *)&LStack176);
    lVar10 = lib::L2CValue::as_number((L2CValue *)&LStack208);
    uVar9 = lib::L2CValue::as_number(aLStack224);
    local_60 = uVar4 & 0xffffffff | lVar10 << 0x20;
    uStack88 = (ulong)uVar9;
    app::lua_bind::KineticModule__add_speed_impl(this->moduleAccessor,(Vector3f *)&local_60);
    lib::L2CValue::~L2CValue(aLStack224);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack208);
    this_01 = &LStack176;
  }
  lib::L2CValue::~L2CValue((L2CValue *)this_01);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack192);
LAB_7100006b40:
  lib::L2CValue::L2CValue((L2CValue *)&local_60,SpecialNSpin_main_loop);
  lua2cpp::L2CFighterCommon::sub_shift_status_main(this,(L2CValue)0xa0,return_value);
  lib::L2CValue::~L2CValue((L2CValue *)&local_60);
  lib::L2CValue::~L2CValue(aLStack160);
  lib::L2CValue::~L2CValue(aLStack144);
  return;
}

