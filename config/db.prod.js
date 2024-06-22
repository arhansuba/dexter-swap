const mongoose = require('mongoose');

// MongoDB connection URL for production environment
const mongoURI = 'mongodb://production-url:27017/mydatabase';

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
    console.log('Connected to MongoDB (Production)');
});

// Exporting the database connection
module.exports = db;
