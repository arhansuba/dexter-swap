const mongoose = require('mongoose');

// MongoDB connection URL for testing environment
const mongoURI = 'mongodb://localhost:27017/testdatabase';

// Mongoose connection setup
mongoose.connect(mongoURI, {
    useNewUrlParser: true,
    useUnifiedTopology: true,
    useCreateIndex: true,
    useFindAndModify: false
});

const db = mongoose.connection;

// Event handlers for database connection
db.on('error', console.error.bind(console, 'MongoDB connection error:'));
db.once('open', () => {
    console.log('Connected to MongoDB (Testing)');
});

// Exporting the database connection
module.exports = db;
