import { h, FunctionalComponent, JSX } from 'preact';
import { useState, useRef, useEffect } from 'preact/hooks';
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

const MIME_TYPES = ['image/gif', 'image/jpeg', 'image/png', 'image/bmp'];

const Home: FunctionalComponent = () => {
    const [file, setFile] = useState<File | null>(null);
    const handleFileChange = (e: JSX.TargetedEvent) => {
        if (!(e.target instanceof HTMLInputElement)) {
            return;
        }

        const files = e.target.files;
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
                requestData: { file, url },
            },
            ''
        );
    };

    useEventListener('dragover', (e) => {
        e.preventDefault();
        if (e.dataTransfer) {
            e.dataTransfer.dropEffect = 'copy';
        }
    });

    useEventListener('drop', (e) => {
        e.preventDefault();
        if (!e.dataTransfer) {
            return;
        }

        const files = e.dataTransfer.files;
        if (files.length > 0 && MIME_TYPES.includes(files[0].type)) {
            setFile(files[0]);
        }

        const url = e.dataTransfer.getData('text').trim();
        if (url !== '') {
            setUrl(url);
        }
    });

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
                                    accept={MIME_TYPES.join(',')}
                                    filenames={[
                                        file?.name ?? 'No file selected',
                                    ]}
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

function useEventListener<K extends keyof WindowEventMap>(
    event: K,
    handler: (event: WindowEventMap[K]) => void
) {
    const handlerRef = useRef<(event: WindowEventMap[K]) => void>();

    useEffect(() => {
        handlerRef.current = handler;
    }, [handler]);

    useEffect(() => {
        const listener = handlerRef.current;
        window.addEventListener(event, listener);
        return () => window.removeEventListener(event, listener);
    }, [event]);
}

export default Home;
