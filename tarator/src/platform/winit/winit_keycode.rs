use winit::event::{VirtualKeyCode, VirtualKeyCode::*};
/// ## get_tr_keycode
/// gets the winit 
pub fn get_tr_keycode(virtual_keycode: &VirtualKeyCode) -> u32 {
    return match virtual_keycode {
        Key1            => TR_1!(),
        Key2            => TR_2!(),
        Key3            => TR_3!(),
        Key4            => TR_4!(),
        Key5            => TR_5!(),
        Key6            => TR_6!(),
        Key7            => TR_7!(),
        Key8            => TR_8!(),
        Key9            => TR_9!(),
        Key0            => TR_0!(),
        A               => TR_A!(),
        B               => TR_B!(),
        C               => TR_C!(),
        D               => TR_D!(),
        E               => TR_E!(),
        F               => TR_F!(),
        G               => TR_G!(),
        H               => TR_H!(),
        I               => TR_I!(),
        J               => TR_J!(),
        K               => TR_K!(),
        L               => TR_L!(),
        M               => TR_M!(),
        N               => TR_N!(),
        O               => TR_O!(),
        P               => TR_P!(),
        Q               => TR_Q!(),
        R               => TR_R!(),
        S               => TR_S!(),
        T               => TR_T!(),
        U               => TR_U!(),
        V               => TR_V!(),
        W               => TR_W!(),
        X               => TR_X!(),
        Y               => TR_Y!(),
        Z               => TR_Z!(),
        Escape          => TR_ESCAPE!(),
        F1              => TR_F1!(),
        F2              => TR_F2!(),
        F3              => TR_F3!(),
        F4              => TR_F4!(),
        F5              => TR_F5!(),
        F6              => TR_F6!(),
        F7              => TR_F7!(),
        F8              => TR_F8!(),
        F9              => TR_F9!(),
        F10             => TR_F10!(),
        F11             => TR_F11!(),
        F12             => TR_F12!(),
        F13             => TR_F13!(),
        F14             => TR_F14!(),
        F15             => TR_F15!(),
        F16             => TR_F16!(),
        F17             => TR_F17!(),
        F18             => TR_F18!(),
        F19             => TR_F19!(),
        F20             => TR_F20!(),
        F21             => TR_F21!(),
        F22             => TR_F22!(),
        F23             => TR_F23!(),
        F24             => TR_F24!(),
        Snapshot        => TR_SNAPSHOT!(),
        Scroll          => TR_SCOLL!(),
        Pause           => TR_PAUSE!(),
        Insert          => TR_INSERT!(),
        Home            => TR_HOME!(),
        Delete          => TR_DELETE!(),
        End             => TR_END!(),
        PageDown        => TR_NEXT!(),
        PageUp          => TR_PRIOR!(),
        Left            => TR_LEFT!(),
        Up              => TR_UP!(),
        Right           => TR_RIGHT!(),
        Down            => TR_DOWN!(),
        Back            => TR_BACK!(),
        Return          => TR_RETURN!(),
        Space           => TR_SPACE!(),
        Compose         => 0,                   // NOT YET DEFINED [TODO]
        Caret           => 0,                   // NOT YET DEFINED [TODO]
        Numlock         => TR_NUMLOCK!(),
        Numpad0         => TR_NUMPAD0!(),
        Numpad1         => TR_NUMPAD1!(),
        Numpad2         => TR_NUMPAD2!(),
        Numpad3         => TR_NUMPAD3!(),
        Numpad4         => TR_NUMPAD4!(),
        Numpad5         => TR_NUMPAD5!(),
        Numpad6         => TR_NUMPAD6!(),
        Numpad7         => TR_NUMPAD7!(),
        Numpad8         => TR_NUMPAD8!(),
        Numpad9         => TR_NUMPAD9!(),
        NumpadAdd       => TR_ADD!(),
        NumpadDivide    => TR_DIVIDE!(),
        NumpadDecimal   => TR_DECIAML!(),
        NumpadComma     => TR_DECIAML!(),
        NumpadEnter     => TR_SEPERATOR!(),
        NumpadEquals    => TR_OEM_NEC_EQUAL!(),
        NumpadMultiply  => TR_MULTIPLY!(),
        NumpadSubtract  => TR_SUBTRACT!(),
        AbntC1          => 0,                   // NOT YET DEFINED [TODO]
        AbntC2          => 0,                   // NOT YET DEFINED [TODO]
        Apostrophe      => TR_OEM_3!(),
        Apps            => TR_APPS!(),
        Asterisk        => TR_OEM_PLUS!(),
        At              => 0,                   // NOT YET DEFINED [TODO]
        Ax              => TR_OEM_AX!(),
        Backslash       => TR_OEM_5!(),
        Calculator      => 0,                   // NOT YET DEFINED [TODO]
        Capital         => 0,                   // NOT YET DEFINED [TODO]
        Colon           => 0,                   // NOT YET DEFINED [TODO]
        Comma           => TR_OEM_COMMA!(),
        Convert         => 0,                   // NOT YET DEFINED [TODO]
        Equals          => 0,                   // NOT YET DEFINED [TODO]                   
        Grave           => 0,                   // NOT YET DEFINED [TODO]
        Kana            => TR_KANA!(),
        Kanji           => TR_KANJI!(),
        LAlt            => TR_MENU!(),
        LBracket        => TR_OEM_4!(),
        LControl        => TR_LCONTROL!(),
        LShift          => TR_LSHIFT!(),
        LWin            => TR_LWIN!(),
        Mail            => TR_LAUNCH_MAIL!(),
        MediaSelect     => TR_LAUNCH_MEDIA_SELECT!(),
        MediaStop       => TR_MEDIA_STOP!(),
        Minus           => TR_OEM_MINUS!(),
        Mute            => TR_VOLUME_MUTE!(),
        MyComputer      => 0,                   // NOT YET DEFINED [TODO]
        NavigateForward => 0,                   // NOT YET DEFINED [TODO]
        NavigateBackward=> 0,                   // NOT YET DEFINED [TODO]
        NextTrack       => TR_MEDIA_NEXT_TRACK!(),
        NoConvert       => TR_NONCONVERT!(),
        OEM102          => TR_OEM_102!(),
        Period          => TR_OEM_PERIOD!(),
        PlayPause       => TR_MEDIA_PLAY_PAUSE!(),
        Plus            => TR_OEM_PLUS!(),
        Power           => 0,                   // NOT YET DEFINED [TODO]
        PrevTrack       => TR_MEDIA_PREV_TRACK!(),
        RAlt            => 0,                   // NOT YET DEFINED [TODO]
        RBracket        => TR_OEM_6!(),
        RControl        => TR_RCONTROL!(),
        RShift          => TR_RSHIFT!(),
        RWin            => TR_RMENU!(),
        Semicolon       =>TR_OEM_1!(),
        Slash           => TR_OEM_2!(),
        Sleep           => TR_SLEEP!(),
        Stop            => TR_MEDIA_STOP!(),
        Sysrq           => 0,                   // NOT YET DEFINED [TODO]
        Tab             => TR_TAB!(),
        Underline       => 0,                   // NOT YET DEFINED [TODO]
        Unlabeled       => 0,                   // NOT YET DEFINED [TODO]
        VolumeDown      => TR_VOLUME_DOWN!(),
        VolumeUp        => TR_VOLUME_UP!(),
        Wake            => 0,                   // NOT YET DEFINED [TODO]
        WebBack         => TR_WEB_BACK!(),
        WebFavorites    => TR_WEB_FAVORITES!(),
        WebForward      => TR_WEB_FORWARD!(),
        WebHome         => TR_WEB_HOME!(),
        WebRefresh      => TR_WEB_REFRESH!(),
        WebSearch       => TR_WEB_SEARCH!(),
        WebStop         => TR_WEB_STOP!(),
        Yen             => 0,                   // NOT YET DEFINED [TODO]
        Copy            => TR_OEM_COPY!(),
        Paste           => 0,                   // NOT YET DEFINED [TODO]
        Cut             => 0                    // NOT YET DEFINED [TODO]
    };
}