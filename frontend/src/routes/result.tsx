import { h, Fragment, FunctionalComponent } from 'preact';
import { useEffect, useState } from 'preact/hooks';
import { route } from 'preact-router';
import {
    Table,
    Image,
    Message,
    Button,
    Icon,
    Section,
    Content,
} from 'preact-bulma';
import { baseroute } from '../baseroute';

type RequestData = {
    file: File | null;
    url: string | null;
};

type ResponseData = {
    ok: boolean;
    error?: string;
    general?: Tag[];
    character?: Tag[];
};

type Tag = {
    name: string;
    score: number;
};

type HistoryState = {
    requestData?: RequestData;
    responseData?: ResponseData | null;
    imageUrl?: string | null;
};

function getState(): HistoryState {
    return history.state;
}

const Result: FunctionalComponent = () => {
    const [responseData, setResponseData] = useState<ResponseData | null>(
        () => getState()?.responseData || null
    );
    const [imageUrl, setImageUrl] = useState<string | null>(
        () => getState()?.imageUrl || null
    );

    useEffect(() => {
        if (responseData) {
            history.replaceState({ responseData, ...history.state }, '');
        }
        if (imageUrl) {
            history.replaceState({ imageUrl, ...history.state }, '');
        }
    }, [responseData, imageUrl]);

    useEffect(() => {
        if (!getState() || !getState()?.requestData) {
            route(`${baseroute}/`, true);
            return;
        }

        const requestData = getState().requestData as RequestData;
        const controller = new AbortController();
        const signal = controller.signal;

        if (!getState().responseData) {
            const body = new FormData();
            if (requestData.file) {
                body.append('file', requestData.file as Blob);
            } else {
                body.append('url', requestData.url as string);
            }

            fetch(process.env.PREACT_APP_API_ENDPOINT as string, {
                method: 'POST',
                body,
                signal,
            })
                .then((response) => response.json())
                .then(setResponseData)
                .catch((err) => {
                    if (err.name === 'AbortError') {
                        return;
                    }
                    setResponseData({
                        ok: false,
                        error: 'Failed to receive response',
                    });
                });
        }

        if (!getState().imageUrl) {
            if (requestData.file && requestData.file.size > 0) {
                const reader = new FileReader();
                reader.onload = () => setImageUrl(reader.result as string);
                signal.onabort = () => reader.abort();
                reader.readAsDataURL(requestData.file);
            } else {
                setImageUrl(requestData.url);
            }
        }

        return () => controller.abort();
    }, []);

    if (!responseData) {
        return <Loader />;
    }

    return (
        <>
            <div className="block">
                <Button
                    color="link"
                    outlined={true}
                    onClick={() => history.back()}
                >
                    <Icon icon="fas fa-chevron-left" />
                    <span>Back</span>
                </Button>
            </div>
            {responseData.ok ? (
                <div className="columns is-centered">
                    <div className="column">
                        {imageUrl && <Image alt="Image" src={imageUrl} />}
                    </div>
                    <div className="column is-narrow">
                        <TagTable
                            category="General tag"
                            list={responseData.general || null}
                        />
                    </div>
                    <div className="column is-narrow">
                        <TagTable
                            category="Character"
                            list={responseData.character || null}
                        />
                    </div>
                </div>
            ) : (
                <div className="container is-max-desktop">
                    <Message title="Error" class="is-danger">
                        {responseData.error}
                    </Message>
                </div>
            )}
        </>
    );
};

const Loader: FunctionalComponent = () => {
    return (
        <div className="is-flex is-flex-direction-column is-justify-content-space-around is-align-items-center">
            <div
                className="block loader is-loading"
                style={{
                    width: '100px',
                    height: '100px',
                }}
            />
            <div className="has-text-centered is-size-5">
                <p>Processing...</p>
                <p>This could take up to 15 seconds</p>
            </div>
        </div>
    );
};

type TagTableProps = {
    category: string;
    list: Tag[] | null;
};

const TagTable: FunctionalComponent<TagTableProps> = ({
    category,
    list,
}: TagTableProps) => (
    <Table narrow={true} hoverable={true} fullWidth={true}>
        <thead>
            <tr>
                <th>{category}</th>
                <th>
                    <span className="is-pulled-right">Score</span>
                </th>
            </tr>
        </thead>
        <tbody>
            {(list &&
                list.map(({ name, score }) => (
                    <tr key={name}>
                        <td>
                            <a
                                href={`https://danbooru.donmai.us/wiki_pages/${name}`}
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                {name}
                            </a>
                        </td>
                        <td>
                            <span className="is-pulled-right">
                                {score.toFixed(3)}
                            </span>
                        </td>
                    </tr>
                ))) || (
                <tr>
                    <td colSpan={2}>
                        <Section>
                            <Content class="has-text-grey has-text-centered">
                                <p>
                                    <Icon icon="far fa-3x fa-frown" />
                                </p>
                                <p>No data</p>
                            </Content>
                        </Section>
                    </td>
                </tr>
            )}
        </tbody>
    </Table>
);

export default Result;
