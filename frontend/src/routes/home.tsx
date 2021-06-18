import { h, FunctionalComponent, JSX } from 'preact';
import { useState } from 'preact/hooks';
import { route } from 'preact-router';
import {
    FileInput,
    Field,
    TextInput,
    Button,
    Control,
    HorizontalGroup,
} from 'preact-bulma';
import { baseroute } from '../baseroute';

const Home: FunctionalComponent = () => {
    const [file, setFile] = useState<File | null>(null);
    const handleFileChange = (e: JSX.TargetedEvent) => {
        if (!(e.target instanceof HTMLInputElement)) {
            return;
        }

        const files: FileList | null = e.target.files;
        if (files && files.length > 0) {
            setFile(files[0]);
        }
    };

    const [url, setUrl] = useState('');
    const handleUrlInput = (e: JSX.TargetedEvent) => {
        if (!(e.target instanceof HTMLInputElement)) {
            return;
        }
        setUrl(e.target.value);
    };

    const handleSubmit = (e: JSX.TargetedEvent) => {
        e.preventDefault();
        route(`${baseroute}/result`);
        history.replaceState(
            {
                requestData: {
                    file,
                    url,
                },
            },
            ''
        );
    };

    return (
        <div className="container is-max-desktop">
            <form onSubmit={handleSubmit}>
                <div className="block">
                    <HorizontalGroup label="Upload">
                        <Field>
                            <Control>
                                <FileInput
                                    label="Choose a file"
                                    icon="fas fa-upload"
                                    color="info"
                                    filename={file?.name ?? 'No file selected'}
                                    onChange={handleFileChange}
                                />
                            </Control>
                        </Field>
                    </HorizontalGroup>
                </div>
                <div className="block">
                    <HorizontalGroup label=" ">or</HorizontalGroup>
                </div>
                <div className="block">
                    <HorizontalGroup label="From URL">
                        <Field>
                            <Control>
                                <TextInput
                                    type="url"
                                    placeholder="URL"
                                    value={url}
                                    onInput={handleUrlInput}
                                />
                            </Control>
                        </Field>
                    </HorizontalGroup>
                </div>
                <div className="field is-grouped is-grouped-right">
                    <Control>
                        <Button color="primary" type="submit">
                            Submit
                        </Button>
                    </Control>
                </div>
            </form>
        </div>
    );
};

export default Home;
