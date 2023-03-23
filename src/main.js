import ruuvi from 'node-ruuvitag';
import fetch from 'node-fetch';

ruuvi.on('found', (tag) => {
    console.log(`Found RuuviTag with id: ${tag.id}`);

    tag.on('updated', async (data) => {
        console.log(`${tag.id}: ${JSON.stringify(data)}`);

        const body = { message: `temperature: ${data.temperature}` };

        try {
            const response = await fetch('http://95.216.207.125:9000/api/chats', {
                method: 'post',
                body: JSON.stringify(body),
                headers: { 'Content-Type': 'application/json' },
            });
            if (!response.ok) {
                const {status, statusText} = response;
                console.error(`Fetch failed with status ${status} ${statusText}`);
            }
        } catch (error) {
            console.error(error);
        }
    });
});
