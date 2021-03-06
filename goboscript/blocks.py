import gobomatic as gm
from typing import Callable

STATEMENT_BLOCKS: dict[str, Callable] = {
    "return": gm.StopThisScript,
    "exit": gm.StopAll,
    "stopotherscripts": gm.StopOtherScripts,
    "if": gm.If,
    "until": gm.Until,
    "repeat": gm.Repeat,
    "forever": gm.Forever,
    "say": gm.Say,
    "sayfor": gm.SayFor,
    "think": gm.Think,
    "thinkfor": gm.ThinkFor,
    "switchcostume": gm.SwitchCostume,
    "nextcostume": gm.NextCostume,
    "switchbackdrop": gm.SwitchBackdrop,
    "nextbackdrop": gm.NextBackdrop,
    "changesize": gm.ChangeSize,
    "setsize": gm.SetSize,
    "cleargraphiceffects": gm.ClearGraphicEffects,
    "setgraphiceffect": gm.SetGraphicEffect,
    "changegraphiceffect": gm.ChangeGraphicEffect,
    "hide": gm.Hide,
    "show": gm.Show,
    "gotofrontback": gm.GotoFrontBack,
    "goforwardbackwardlayers": gm.GoForwardBackwardLayers,
    "eraseall": gm.EraseAll,
    "stamp": gm.Stamp,
    "pendown": gm.PenDown,
    "penup": gm.PenUp,
    "setpencolor": gm.SetPenColor,
    "changepencolorparam": gm.ChangePenColorParam,
    "setpencolorparam": gm.SetPenColorParam,
    "changepensize": gm.ChangePenSize,
    "setpensize": gm.SetPenSize,
    "ask": gm.Ask,
    "setdraggable": gm.SetDraggable,
    "setnotdraggable": gm.SetNotDraggable,
    "resettimer": gm.ResetTimer,
    "move": gm.Move,
    "turnright": gm.TurnRight,
    "turnleft": gm.TurnLeft,
    "gotosprite": gm.GotoSprite,
    "goto": gm.Goto,
    "glidetosprite": gm.GlideToSprite,
    "glide": gm.Glide,
    "point": gm.Point,
    "pointtowards": gm.PointTowards,
    "changex": gm.ChangeX,
    "changey": gm.ChangeY,
    "setx": gm.SetX,
    "sety": gm.SetY,
    "ifonedgebounce": gm.IfOnEdgeBounce,
    "setrotationstyle": gm.SetRotationStyle,
    "broadcast": gm.Broadcast,
    "broadcastandwait": gm.BroadcastAndWait,
    "wait": gm.Wait,
    "waituntil": gm.WaitUntil,
    "createclone": gm.CreateClone,
    "deletethisclone": gm.DeleteThisClone,
    "stop": gm.Stop,
    "setvariable": gm.SetVariable,
    "changevariable": gm.ChangeVariable,
    "showvariable": gm.ShowVariable,
    "hidevariable": gm.HideVariable,
    "addtolist": gm.AddToList,
    "deleteoflist": gm.DeleteOfList,
    "deletealloflist": gm.DeleteAllOfList,
    "insertatlist": gm.InsertAtList,
    "replaceitemoflist": gm.ReplaceItemOfList,
    "showlist": gm.ShowList,
    "hidelist": gm.HideList,
    "playsounduntildone": gm.PlaySoundUntilDone,
    "startsound": gm.StartSound,
    "stopallsounds": gm.StopAllSounds,
    "setsoundeffect": gm.SetSoundEffect,
    "changesoundeffect": gm.ChangeSoundEffect,
    "clearsoundeffects": gm.ClearSoundEffects,
    "changevolume": gm.ChangeVolume,
    "setvolume": gm.SetVolume,
    "setcoloreffect": gm.SetColorEffect,
    "setfisheyeeffect": gm.SetFisheyeEffect,
    "setwhirleffect": gm.SetWhirlEffect,
    "setpixelateeffect": gm.SetPixelateEffect,
    "setmosaiceffect": gm.SetMosaicEffect,
    "setbrightnesseffect": gm.SetBrightnessEffect,
    "setghosteffect": gm.SetGhostEffect,
    "changecoloreffect": gm.ChangeColorEffect,
    "changefisheyeeffect": gm.ChangeFisheyeEffect,
    "changewhirleffect": gm.ChangeWhirlEffect,
    "changepixelateeffect": gm.ChangePixelateEffect,
    "changemosaiceffect": gm.ChangeMosaicEffect,
    "changebrightnesseffect": gm.ChangeBrightnessEffect,
    "changeghosteffect": gm.ChangeGhostEffect,
    "gotofront": gm.GotoFront,
    "gotoback": gm.GotoBack,
    "goforward": gm.GoForward,
    "gobackward": gm.GoBackward,
    "changepenhue": gm.ChangePenHue,
    "changepensaturation": gm.ChangePenSaturation,
    "changepenbrightness": gm.ChangePenBrightness,
    "changepentransparency": gm.ChangePenTransparency,
    "setpenhue": gm.SetPenHue,
    "setpensaturation": gm.SetPenSaturation,
    "setpenbrightness": gm.SetPenBrightness,
    "setpentransparency": gm.SetPenTransparency,
    "gotomousepointer": gm.GotoMousePointer,
    "gotorandomposition": gm.GotoRandomPosition,
    "glidetomousepointer": gm.GlideToMousePointer,
    "glidetorandomposition": gm.GlideToRandomPosition,
    "pointtowardsmousepointer": gm.PointTowardsMousePointer,
    "setrotationstyleleftright": gm.SetRotationStyleLeftRight,
    "setrotationstyleallaround": gm.SetRotationStyleAllAround,
    "setrotationstyledontrotate": gm.SetRotationStyleDontRotate,
    "sa_breakpoint": gm.SA_Breakpoint,
    "sa_log": gm.SA_Log,
    "sa_warn": gm.SA_Warn,
    "sa_error": gm.SA_Error,
    "setpitcheffect": gm.SetPitchEffect,
    "changepitcheffect": gm.ChangePitchEffect,
    "setpaneffect": gm.SetPanEffect,
    "changepaneffect": gm.ChangePanEffect,
}


REPORTER_BLOCKS: dict[str, Callable] = {
    "mousedown": gm.MouseDown,
    "xposition": gm.XPosition,
    "yposition": gm.YPosition,
    "direction": gm.Direction,
    "costumenumber": gm.CostumeNumber,
    "costumename": gm.CostumeName,
    "backdropnumber": gm.BackdropNumber,
    "backdropname": gm.BackdropName,
    "size": gm.Size,
    "add": gm.Add,
    "sub": gm.Sub,
    "mul": gm.Mul,
    "div": gm.Div,
    "eq": gm.Eq,
    "lt": gm.Lt,
    "gt": gm.Gt,
    "random": gm.Random,
    "join": gm.Join,
    "letter": gm.Letter,
    "length": gm.Length,
    "mod": gm.Mod,
    "round": gm.Round,
    "mathop": gm.MathOp,
    "distanceto": gm.DistanceTo,
    "answer": gm.Answer,
    "mousex": gm.MouseX,
    "mousey": gm.MouseY,
    "loudness": gm.Loudness,
    "timer": gm.Timer,
    "current": gm.Current,
    "username": gm.Username,
    "itemoflist": gm.ItemOfList,
    "indexoflist": gm.IndexOfList,
    "lengthoflist": gm.LengthOfList,
    "listcontainsitem": gm.ListContainsItem,
    "volume": gm.Volume,
    "abs": gm.Abs,
    "floor": gm.Floor,
    "ceiling": gm.Ceiling,
    "sqrt": gm.Sqrt,
    "sin": gm.Sin,
    "cos": gm.Cos,
    "tan": gm.Tan,
    "asin": gm.Asin,
    "acos": gm.Acos,
    "atan": gm.Atan,
    "ln": gm.Ln,
    "log": gm.Log,
    "antiln": gm.AntiLn,
    "antilog": gm.AntiLog,
    "currentyear": gm.CurrentYear,
    "currentmonth": gm.CurrentMonth,
    "currentdate": gm.CurrentDate,
    "currentdayofweek": gm.CurrentDayofweek,
    "currenthour": gm.CurrentHour,
    "currentminute": gm.CurrentMinute,
    "currentsecond": gm.CurrentSecond,
}
