/* eslint react-hooks/exhaustive-deps: off */
import React, { useEffect, useState } from "react";
import { ColorfulMessage } from "./components/ColorfulMessage";

const App = () => {
    console.log("最初");
    const [num, setNum] = useState(0);
    const [faceShowFlag, setFaseShowFlag] = useState(false);

    const onClickCountUp = () => {
        setNum(num + 1)
    };

    const onClickSwitchShowFlag = () => {
        setFaseShowFlag(!faceShowFlag);
    };

    useEffect(() => {
        if (num > 0 && num % 3 === 0) {
            faceShowFlag || setFaseShowFlag(true);
        } else {
            faceShowFlag && setFaseShowFlag(false);
        }
    }, [num]);

    return (
        <>
            <h1 style={{ color: 'red' }}>こんにちは!</h1>
            <ColorfulMessage color="blue">お元気ですか?</ColorfulMessage>
            <ColorfulMessage color="pink">元気です!</ColorfulMessage>
            <button onClick={onClickCountUp}>カウントアップ</button>
            <br />
            <button onClick={onClickSwitchShowFlag}>on/off</button>
            <p>{num}</p>
            {faceShowFlag && <p>＼(^o^)／</p>}
        </>
    );
};

export default App;