const fastify = require('fastify')({ logger: true });
const path = require('path');

// 静的ファイルの配信
fastify.register(require('@fastify/static'), {
  root: path.join(__dirname, 'static'),
  prefix: '/', // 静的ファイルのURLパスのプレフィックス
});

// ルート

fastify.get('/', async (request, reply) => {
    try {
      if (!reply.sent) {
        reply.sendFile('index.html');
      }
    } catch (err) {
        console.log(err);
      if (!reply.sent) {
        reply.status(500).send({ error: 'Internal Server Error' });
      }
    }

    return reply;
  });
  

// サーバーの起動
const start = async () => {
  try {
    await fastify.listen({ port: 3000 });
    fastify.log.info(`Server listening on ${fastify.server.address().port}`);
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();
