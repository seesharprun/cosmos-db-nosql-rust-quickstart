const socket = io({
    forceNew: false,
    reconnection: true,
    transports: ['websocket', 'polling'],
    reconnectionAttempts: 1,
    reconnectionDelayMax: 2000
});

socket.on('connect', () => {
    console.log('Connected!');
    $('#console').html('&#x200B;'); // Zero-width space
    socket.emit('start', null);
});
socket.on('new_message', (data) => {
    console.dir(data);
    $('#console').append(`${data}\n`);
});
socket.on('disconnect', () => {
    console.log('Disconnected!');
});

$('#runAgainButton').click(() => {
    $('#console').html('&#x200B;'); // Zero-width space
    socket.emit('start', null);
});