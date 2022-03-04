const ruuvi = require('node-ruuvitag');
const fetch = require('node-fetch');

ruuvi.on('found', (tag) => {
  console.log(`Found RuuviTag with id: ${tag.id}`);

  tag.on('updated', (data) => {
    console.log(`${tag.id}: ${JSON.stringify(data)}`);

    const body = { message: `temperature: ${data.temperature}` };

    fetch('http://95.216.154.69:9000/api/chats', {
      method: 'post',
      body: JSON.stringify(body),
      headers: { 'Content-Type': 'application/json' },
    });
  });
});
