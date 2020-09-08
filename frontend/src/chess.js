const SIZE = 15
const SIZE_PX = 400

function VerticalLine(props) {
    return <div class="vertical line" style={{
        left: (props.spacing + props.spacing * props.x) + "px",
        top: props.spacing + "px",
        height: (SIZE_PX - props.spacing * 2) + "px",
    }}/>
}

function HorizontalLine(props) {
    return <div class="horizontal line" style={{
        top: (props.spacing + props.spacing * props.y) + "px",
        left: props.spacing + "px",
        width: (SIZE_PX - props.spacing * 2) + "px",
    }}/>
}

function Chess(props) {
    return <div class="chess" style={{
        width: props.spacing,
        height: props.spacing,
        top: (props.spacing / 2 + props.spacing * props.y - 1) + "px",
        left: (props.spacing / 2 + props.spacing * props.x - 1) + "px",
    }}/>
}

export default function ChessBoard(props) {
    let spacing = SIZE_PX / (SIZE + 1)
    let lines = []
    for (let i = 0; i < SIZE; i++) {
        lines.push(<VerticalLine x={i} spacing={spacing}/>)
        lines.push(<HorizontalLine y={i} spacing={spacing}/>)
    }
    let chesses = []
    for (let y = 0; y < SIZE; y++) {
        for (let x = 0; x < SIZE; x++) {
            chesses.push(<Chess x={x} y={y} spacing={spacing}/>)
        }
    }

    console.log(lines)
    return <div class="chess-board" style={{width: SIZE_PX, height: SIZE_PX}}>
        {lines}
        {chesses}
    </div>
}
