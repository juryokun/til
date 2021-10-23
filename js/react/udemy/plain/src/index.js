import "./styles.css";

const onClickAdd = () => {
    const inputText = document.getElementById("add-text").value;
    document.getElementById("add-text").value = "";

    createIncompleteList(inputText);
}

// 未完了リストから削除
const deleteFromIncompleteList = (target) => {
    document.getElementById("incomplete-list").removeChild(target);
}

// 未完了リスト追加
const createIncompleteList = (text) => {
    // div生成
    const div = document.createElement("div");
    div.className = "list-row";

    // li作成
    const li = document.createElement("li");
    li.innerText = text;

    // button生成
    const completeButton = document.createElement("button");
    completeButton.innerText = "完了";
    completeButton.addEventListener("click", () => {
        // 未完了リストから削除
        deleteFromIncompleteList(completeButton.parentNode);

        //完了リストに追加
        const addTarget = completeButton.parentNode;
        const text = addTarget.firstElementChild.innerText;
        addTarget.textContent = null;

        const li = document.createElement("li");
        li.innerText = text;

        const backButton = document.createElement("button");
        backButton.innerText = "戻す";
        backButton.addEventListener("click", () => {
            // 完了リストから削除
            const deleteTarget = backButton.parentNode;
            document.getElementById("complete-list").removeChild(deleteTarget);

            const text = backButton.parentNode.firstElementChild.innerText;
            createIncompleteList(text);
        })

        addTarget.appendChild(li);
        addTarget.appendChild(backButton);

        document.getElementById("complete-list").appendChild(addTarget);

    });

    const deleteButton = document.createElement("button");
    deleteButton.innerText = "削除";
    deleteButton.addEventListener("click", () => {
        // 未完了リストから削除
        deleteFromIncompleteList(deleteButton.parentNode);
    });

    // divの子要素にli追加
    div.appendChild(li);
    div.appendChild(completeButton);
    div.appendChild(deleteButton);

    // 未完了リストに追加
    document.getElementById("incomplete-list").appendChild(div);
}
document
    .getElementById("add-button")
    .addEventListener("click", () => onClickAdd());