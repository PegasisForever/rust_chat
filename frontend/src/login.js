import {Component} from "preact"
import {setName} from "./tools"

export default class LoginPage extends Component {
    state = {name: ""}

    onSubmit = e => {
        e.preventDefault()
        setName(this.state.name)
        this.props.onUpdateLogin()
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
                <input required="required" type="text" id="name-input" value={name} onInput={this.onInput}/>
                <button type="submit">Ok</button>
            </form>
        </div>
    }
}
