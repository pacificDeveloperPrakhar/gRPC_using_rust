var PROTO_PATH = __dirname + '/user.proto';
var grpc = require('@grpc/grpc-js');
var protoLoader = require('@grpc/proto-loader');
// Suggested options for similarity to existing grpc.load behavior
var packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {keepCase: true,
     longs: String,
     enums: String,
     defaults: true,
     oneofs: true
    });
var protoDescriptor = grpc.loadPackageDefinition(packageDefinition);
// The protoDescriptor object has the full package hierarchy
var user_descriptor = protoDescriptor.users;
console.log(user_descriptor)
// --------------------
// In-memory datastore
// --------------------
let users = [];
let nextId = 1;

// --------------------
// RPC HANDLERS
// --------------------
const handlers = {
  // stream User
  GetUsers: (call) => {
    users.forEach((user) => call.write(user));
    call.end();
  },

  // AddUser(AddUser) → User
  AddUser: (call, callback) => {
    const input = call.request;

    const newUser = {
      id: nextId++,
      name: input.name,
      age: input.age,
      is_active: "ONLINE",
    };

    users.push(newUser);
    callback(null, newUser);
  },

  // UpdateUser(UpdateUser) → User
  UpdateUser: (call, callback) => {
    const update = call.request;
    const user = users.find((u) => u.id === update.id);

    if (!user) {
      return callback({
        code: grpc.status.NOT_FOUND,
        message: "User not found",
      });
    }

    user.name = update.name ?? user.name;
    user.age = update.age ?? user.age;
    user.is_active = update.is_active ?? user.is_active;

    callback(null, user);
  },
};

const server = new grpc.Server();
server.addService(user_descriptor.UserGuide.service, handlers);

server.bindAsync(
  "0.0.0.0:50051",
  grpc.ServerCredentials.createInsecure(),
  () => {
    console.log("gRPC server running at port 50051");
    server.start();
  }
);