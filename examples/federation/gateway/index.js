const { ApolloServer } = require('apollo-server');
const { ApolloGateway } = require('@apollo/gateway');

const gateway = new ApolloGateway({
    serviceList: [
        { name: 'accounts', url: 'http://localhost:4001' },
        { name: 'products', url: 'http://localhost:4003' },
        { name: 'reviews', url: 'http://localhost:4005' },
    ],
});

const server = new ApolloServer({
    gateway,
    subscriptions: false,
    tracing: true,
});

server.listen().then(({ url }) => {
    console.log(`🚀 Server ready at ${url}`);
});
