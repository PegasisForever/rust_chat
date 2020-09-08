function SelectItem(props) {
    return <div class={props.isSelected ? "selected" : ""} onclick={props.onClick}>
        <img src={props.src} alt=""/>
        <span>{props.text}</span>
    </div>
}

export default function ChessSelect(props) {
    return <div class="chess-select-bar">
        <SelectItem src={"select_black.svg"} text={"Black"} isSelected={props.isBlack === true}
                    onClick={()=>props.onChange(true)}/>
        <SelectItem src={"select_white.svg"} text={"White"} isSelected={props.isBlack === false}
                    onClick={()=>props.onChange(false)}/>
        <SelectItem src={"select_none.svg"} text={"Remove"} isSelected={props.isBlack === null}
                    onClick={()=>props.onChange(null)}/>
    </div>
}
