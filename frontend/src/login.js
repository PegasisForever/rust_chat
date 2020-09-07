import {Component} from "preact"

export default class LoginPage extends Component {
    state = {name: ""}

    onSubmit = e => {
        e.preventDefault()
        console.log(this.state.name)
    }

    onInput = e => {
        this.setState({
            name: e.target.value,
        })
    }

    render(_, {name}) {
        return <div class={"center-child"}>
            <form onSubmit={this.onSubmit}>
                <label for="name-input">Name:</label>
                <input type="text" id="name-input" value={name} onInput={this.onInput}/>
                <button type="submit">Ok</button>
            </form>
        </div>
    }
}
