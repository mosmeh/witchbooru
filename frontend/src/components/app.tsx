import { h, FunctionalComponent } from 'preact';
import { Router, Route } from 'preact-router';
import { Container, Title } from 'preact-bulma';
import Home from '../routes/home';
import Result from '../routes/result';

const App: FunctionalComponent = () => {
    return (
        <div id="app">
            <Container>
                <Title>Witchbooru</Title>
                <Router>
                    <Route path="/" component={Home} />
                    <Route path="/result" component={Result} />
                </Router>
            </Container>
        </div>
    );
};

export default App;
