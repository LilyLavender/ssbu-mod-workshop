
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_710001ca30(L2CValue *param_1,long param_2,L2CValue *param_3)

{
  int iVar1;
  int iVar2;
  ulong uVar3;
  Hash40 HVar4;
  L2CValue *pLVar5;
  Fighter *pFVar6;
  L2CValue *in_x8;
  L2CValue *return_value;
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
  L2CValue *return_value_14;
  L2CValue *return_value_15;
  L2CValue *return_value_16;
  L2CValue *return_value_17;
  L2CValue *return_value_18;
  L2CValue *return_value_19;
  L2CValue *return_value_20;
  L2CValue *return_value_21;
  L2CValue *extraout_x8;
  L2CValue *return_value_22;
  L2CValue *return_value_23;
  L2CValue *return_value_24;
  L2CValue *return_value_25;
  L2CValue *extraout_x8_00;
  L2CValue *extraout_x8_01;
  L2CValue *return_value_26;
  L2CValue *return_value_27;
  L2CValue *return_value_28;
  L2CValue *return_value_29;
  L2CValue *return_value_30;
  L2CValue *return_value_31;
  L2CValue *return_value_32;
  L2CValue *return_value_33;
  L2CValue *return_value_34;
  L2CValue *return_value_35;
  L2CValue *return_value_36;
  L2CValue *return_value_37;
  L2CValue *return_value_38;
  BattleObjectModuleAccessor **ppBVar7;
  float fVar8;
  L2CValue aLStack448 [16];
  L2CValue aLStack432 [16];
  L2CValue aLStack416 [16];
  L2CValue aLStack400 [16];
  L2CValue aLStack384 [16];
  L2CValue aLStack368 [16];
  L2CValue aLStack352 [16];
  L2CValue aLStack336 [16];
  L2CValue aLStack320 [16];
  L2CValue aLStack304 [16];
  L2CValue aLStack288 [16];
  L2CValue aLStack272 [16];
  L2CValue aLStack256 [16];
  L2CValue aLStack240 [16];
  L2CValue aLStack224 [16];
  L2CValue aLStack208 [16];
  L2CValue aLStack192 [16];
  L2CValue aLStack176 [16];
  L2CValue LStack160;
  L2CValue aLStack144 [16];
  L2CValue aLStack128 [16];
  L2CValue aLStack112 [16];
  L2CValue LStack96;
  
  pLVar5 = aLStack448;
  lib::L2CValue::L2CValue(aLStack112,0,in_x8);
  lib::L2CValue::L2CValue(aLStack128,0,return_value);
  lib::L2CValue::L2CValue(aLStack144,0,return_value_00);
  lib::L2CValue::L2CValue((L2CValue *)&LStack96,false);
  uVar3 = lib::L2CValue::operator==(param_3,(L2CValue *)&LStack96);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
  if ((uVar3 & 1) == 0) {
    lib::L2CValue::L2CValue
              ((L2CValue *)&LStack96,
               _FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_HOP_COUNT,
               (L2CValue *)0x4a80);
    iVar1 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
    ppBVar7 = (BattleObjectModuleAccessor **)(param_2 + 0x40);
    app::lua_bind::WorkModule__inc_int_impl(*ppBVar7,iVar1);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    lib::L2CValue::L2CValue
              ((L2CValue *)&LStack96,_FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE,
               (L2CValue *)0x4a84);
    iVar1 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
    app::lua_bind::WorkModule__dec_int_impl(*ppBVar7,iVar1);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    HVar4 = app::lua_bind::MotionModule__motion_kind_impl(*ppBVar7);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,HVar4);
    lib::L2CValue::operator=(aLStack144,(L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    lib::L2CValue::L2CValue
              (aLStack176,_FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE,
               (L2CValue *)0x4a84);
    iVar1 = lib::L2CValue::as_integer(aLStack176);
    iVar1 = app::lua_bind::WorkModule__get_int_impl(*ppBVar7,iVar1);
    lib::L2CValue::L2CValue((L2CValue *)&LStack160,iVar1,return_value_02);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,0,return_value_03);
    uVar3 = lib::L2CValue::operator<=((L2CValue *)&LStack160,(L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack160);
    lib::L2CValue::~L2CValue(aLStack176);
    if ((uVar3 & 1) != 0) {
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,5,return_value_04);
      lib::L2CValue::L2CValue
                ((L2CValue *)&LStack160,_FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE ,
                 (L2CValue *)0x4a84);
      iVar1 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
      iVar2 = lib::L2CValue::as_integer((L2CValue *)&LStack160);
      app::lua_bind::WorkModule__set_int_impl(*ppBVar7,iVar1,iVar2);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack160);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::L2CValue
                ((L2CValue *)&LStack160,
                 _FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER,
                 (L2CValue *)0x4a88);
      iVar1 = lib::L2CValue::as_integer((L2CValue *)&LStack160);
      iVar1 = app::lua_bind::WorkModule__get_int_impl(*ppBVar7,iVar1);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,iVar1,return_value_05);
      lib::L2CValue::operator=(aLStack112,(L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack160);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,0,return_value_06);
      uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      if ((uVar3 & 1) == 0) {
        lib::L2CValue::L2CValue((L2CValue *)&LStack96,1,return_value_07);
        uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
        lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
        if ((uVar3 & 1) == 0) {
          lib::L2CValue::L2CValue((L2CValue *)&LStack96,2,return_value_09);
          uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
          lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
          if ((uVar3 & 1) == 0) {
            lib::L2CValue::L2CValue((L2CValue *)&LStack96,3,return_value_12);
            uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
            lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
            if ((uVar3 & 1) == 0) {
              lib::L2CValue::L2CValue((L2CValue *)&LStack96,4,return_value_15);
              uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
              lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
              if ((uVar3 & 1) == 0) {
                lib::L2CValue::L2CValue((L2CValue *)&LStack96,5,return_value_18);
                uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
                lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
                if ((uVar3 & 1) == 0) {
                  lib::L2CValue::L2CValue((L2CValue *)&LStack96,6,return_value_27);
                  uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
                  lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
                  if ((uVar3 & 1) == 0) {
                    lib::L2CValue::L2CValue((L2CValue *)&LStack96,7,return_value_29);
                    uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
                    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
                    if ((uVar3 & 1) == 0) {
                      lib::L2CValue::L2CValue((L2CValue *)&LStack96,8,return_value_32);
                      uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
                      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
                      if ((uVar3 & 1) == 0) {
                        lib::L2CValue::L2CValue((L2CValue *)&LStack96,9,return_value_35);
                        uVar3 = lib::L2CValue::operator==(aLStack112,(L2CValue *)&LStack96);
                        lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
                        if ((uVar3 & 1) == 0) {
                          lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x1599968419);
                          HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                          iVar1 = app::lua_bind::SoundModule__play_se_impl
                                            (*ppBVar7,HVar4,true,false,false,false,0);
                          lib::L2CValue::L2CValue(aLStack448,iVar1,return_value_38);
                        }
                        else {
                          lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x1599968419);
                          HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                          iVar1 = app::lua_bind::SoundModule__play_se_impl
                                            (*ppBVar7,HVar4,true,false,false,false,0);
                          lib::L2CValue::L2CValue(aLStack432,iVar1,return_value_37);
                          pLVar5 = aLStack432;
                        }
                      }
                      else {
                        lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x15009fd5a3);
                        HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                        iVar1 = app::lua_bind::SoundModule__play_se_impl
                                          (*ppBVar7,HVar4,true,false,false,false,0);
                        lib::L2CValue::L2CValue(aLStack416,iVar1,return_value_36);
                        pLVar5 = aLStack416;
                      }
                    }
                    else {
                      lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x15009fd5a3);
                      HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                      iVar1 = app::lua_bind::SoundModule__play_se_impl
                                        (*ppBVar7,HVar4,true,false,false,false,0);
                      lib::L2CValue::L2CValue(aLStack400,iVar1,return_value_33);
                      pLVar5 = aLStack400;
                    }
                  }
                  else {
                    lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x119fdc76fa);
                    uVar3 = lib::L2CValue::operator==(aLStack144,(L2CValue *)&LStack96);
                    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
                    if ((uVar3 & 1) == 0) {
                      lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x157798e535);
                      HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                      iVar1 = app::lua_bind::SoundModule__play_se_impl
                                        (*ppBVar7,HVar4,true,false,false,false,0);
                      lib::L2CValue::L2CValue(aLStack384,iVar1,return_value_34);
                      pLVar5 = aLStack384;
                    }
                    else {
                      lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x1896dcd23e);
                      HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                      iVar1 = app::lua_bind::SoundModule__play_se_impl
                                        (*ppBVar7,HVar4,true,false,false,false,0);
                      lib::L2CValue::L2CValue(aLStack368,iVar1,return_value_30);
                      pLVar5 = aLStack368;
                    }
                  }
                }
                else {
                  lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x119fdc76fa);
                  uVar3 = lib::L2CValue::operator==(aLStack144,(L2CValue *)&LStack96);
                  lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
                  if ((uVar3 & 1) == 0) {
                    lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x159020c832);
                    HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                    iVar1 = app::lua_bind::SoundModule__play_se_impl
                                      (*ppBVar7,HVar4,true,false,false,false,0);
                    lib::L2CValue::L2CValue(aLStack352,iVar1,return_value_31);
                    pLVar5 = aLStack352;
                  }
                  else {
                    lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x187603a50d);
                    HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                    iVar1 = app::lua_bind::SoundModule__play_se_impl
                                      (*ppBVar7,HVar4,true,false,false,false,0);
                    lib::L2CValue::L2CValue(aLStack336,iVar1,return_value_28);
                    pLVar5 = aLStack336;
                  }
                }
              }
              else {
                lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x15f0e741d7);
                HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                iVar1 = app::lua_bind::SoundModule__play_se_impl
                                  (*ppBVar7,HVar4,true,false,false,false,0);
                lib::L2CValue::L2CValue(aLStack320,iVar1,return_value_19);
                pLVar5 = aLStack320;
              }
            }
            else {
              lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x119fdc76fa);
              uVar3 = lib::L2CValue::operator==(aLStack144,(L2CValue *)&LStack96);
              lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
              if ((uVar3 & 1) == 0) {
                lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x1587e07141);
                HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                iVar1 = app::lua_bind::SoundModule__play_se_impl
                                  (*ppBVar7,HVar4,true,false,false,false,0);
                lib::L2CValue::L2CValue(aLStack304,iVar1,return_value_20);
                pLVar5 = aLStack304;
              }
              else {
                lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x188ed7a452);
                HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
                iVar1 = app::lua_bind::SoundModule__play_se_impl
                                  (*ppBVar7,HVar4,true,false,false,false,0);
                lib::L2CValue::L2CValue(aLStack288,iVar1,return_value_16);
                pLVar5 = aLStack288;
              }
            }
          }
          else {
            lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x119fdc76fa);
            uVar3 = lib::L2CValue::operator==(aLStack144,(L2CValue *)&LStack96);
            lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
            if ((uVar3 & 1) == 0) {
              lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x1587e07141);
              HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
              iVar1 = app::lua_bind::SoundModule__play_se_impl
                                (*ppBVar7,HVar4,true,false,false,false,0);
              lib::L2CValue::L2CValue(aLStack272,iVar1,return_value_17);
              pLVar5 = aLStack272;
            }
            else {
              lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x188ed7a452);
              HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
              iVar1 = app::lua_bind::SoundModule__play_se_impl
                                (*ppBVar7,HVar4,true,false,false,false,0);
              lib::L2CValue::L2CValue(aLStack256,iVar1,return_value_13);
              pLVar5 = aLStack256;
            }
          }
        }
        else {
          lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x119fdc76fa);
          uVar3 = lib::L2CValue::operator==(aLStack144,(L2CValue *)&LStack96);
          lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
          if ((uVar3 & 1) == 0) {
            lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x159020c832);
            HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
            iVar1 = app::lua_bind::SoundModule__play_se_impl
                              (*ppBVar7,HVar4,true,false,false,false,0);
            lib::L2CValue::L2CValue(aLStack240,iVar1,return_value_14);
            pLVar5 = aLStack240;
          }
          else {
            lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x187603a50d);
            HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
            iVar1 = app::lua_bind::SoundModule__play_se_impl
                              (*ppBVar7,HVar4,true,false,false,false,0);
            lib::L2CValue::L2CValue(aLStack224,iVar1,return_value_10);
            pLVar5 = aLStack224;
          }
        }
      }
      else {
        lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x119fdc76fa);
        uVar3 = lib::L2CValue::operator==(aLStack144,(L2CValue *)&LStack96);
        lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
        if ((uVar3 & 1) == 0) {
          lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x157798e535);
          HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
          iVar1 = app::lua_bind::SoundModule__play_se_impl(*ppBVar7,HVar4,true,false,false,false, 0);
          lib::L2CValue::L2CValue(aLStack208,iVar1,return_value_11);
          pLVar5 = aLStack208;
        }
        else {
          lib::L2CValue::L2CValue((L2CValue *)&LStack96,0x1896dcd23e);
          HVar4 = lib::L2CValue::as_hash((L2CValue *)&LStack96);
          iVar1 = app::lua_bind::SoundModule__play_se_impl(*ppBVar7,HVar4,true,false,false,false, 0);
          lib::L2CValue::L2CValue(aLStack192,iVar1,return_value_08);
          pLVar5 = aLStack192;
        }
      }
      lib::L2CValue::~L2CValue(pLVar5);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::L2CValue
                ((L2CValue *)&LStack96,
                 _FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER,
                 (L2CValue *)0x4a88);
      iVar1 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
      app::lua_bind::WorkModule__inc_int_impl(*ppBVar7,iVar1);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    }
    pLVar5 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)(param_2 + 200),0x16);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,_SITUATION_KIND_GROUND,return_value_21);
    uVar3 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack96);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
    return_value_26 = extraout_x8;
    if ((uVar3 & 1) != 0) {
      lib::L2CValue::L2CValue
                ((L2CValue *)&LStack160,
                 _FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER,
                 (L2CValue *)0x4a7c);
      iVar1 = lib::L2CValue::as_integer((L2CValue *)&LStack160);
      fVar8 = (float)app::lua_bind::WorkModule__get_float_impl(*ppBVar7,iVar1);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,fVar8,return_value_22);
      lib::L2CValue::operator=(aLStack128,(L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack160);
      fVar8 = (float)app::lua_bind::MotionModule__rate_impl(*ppBVar7);
      lib::L2CValue::L2CValue((L2CValue *)&LStack160,fVar8,return_value_23);
      lib::L2CValue::operator-(aLStack128,(L2CValue *)&LStack160,&LStack96);
      lib::L2CValue::operator=(aLStack128,(L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack160);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,0.0,return_value_24);
      lib::L2CValue::operator+(aLStack128,(L2CValue *)&LStack96,&LStack160);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::L2CValue
                ((L2CValue *)&LStack96,
                 _FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER,
                 (L2CValue *)0x4a7c);
      fVar8 = (float)lib::L2CValue::as_number((L2CValue *)&LStack160);
      iVar1 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
      app::lua_bind::WorkModule__set_float_impl(*ppBVar7,fVar8,iVar1);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack160);
      lib::L2CValue::L2CValue((L2CValue *)&LStack96,0,return_value_25);
      uVar3 = lib::L2CValue::operator<=(aLStack128,(L2CValue *)&LStack96);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
      return_value_26 = extraout_x8_00;
      if ((uVar3 & 1) != 0) {
        pLVar5 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)(param_2 + 200),4);
        pFVar6 = (Fighter *)lib::L2CValue::as_pointer(pLVar5);
        app::FighterSpecializer_Metaknight::set_special_n_ground_effect(pFVar6);
        return_value_26 = extraout_x8_01;
      }
    }
    lib::L2CValue::L2CValue(param_1,0,return_value_26);
  }
  else {
    lib::L2CValue::L2CValue(param_1,0,return_value_01);
  }
  lib::L2CValue::~L2CValue(aLStack144);
  lib::L2CValue::~L2CValue(aLStack128);
  lib::L2CValue::~L2CValue(aLStack112);
  return;
}

