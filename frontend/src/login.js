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
            <form class={"login-form"} onSubmit={this.onSubmit}>
                <h2>Name:</h2>
                <input required type="text" value={name} onInput={this.onInput}/>
                <button type="submit">Login</button>
            </form>
        </div>
    }
}
