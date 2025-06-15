#include "socket.hpp"
#include <sys/un.h>
#include <sys/socket.h>
#include <unistd.h>
#include <assert.h>

Socket::Socket(std::string const &path)
{
    assert(path.size() > 0);
    assert(path.size() == path.length());
    log.configure("%s  ", __func__);

    fd = socket(AF_LOCAL, SOCK_STREAM, 0);
    if (fd < 0)
    {
        log.error("Could not create socket on %s", path.c_str());
        exit(1);
    }

    //(void)fcntl(fd, F_SETFD, FD_CLOEXEC); // What for?
    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(struct sockaddr_un));
    addr.sun_family = AF_LOCAL;

    if (path.size() >= sizeof(addr.sun_path) - 1)
    {
        log.error("Couldn't create socket connection, because path length exceeds maximum %s", path.c_str());
        exit(1);
    }
    path.copy(addr.sun_path, path.size());

    // Unix sockets beginning with a null character map to the invisible unix socket space.
  	// Since Strings that begin with a null character a difficult to handle, use % instead
  	// and translate % to the null character here.
    if (addr.sun_path[0] == '%') addr.sun_path[0] = '\0';

    int rc = connect(fd, reinterpret_cast<struct sockaddr *>(&addr),
                     sizeof(addr.sun_path) - 1);

    if (rc < 0)
    {
        log.error("Failed to connect to socket on %s (rc=%d)", addr.sun_path, rc);
        exit(1);
    }
}

bool Socket::write(std::vector<uint8_t> const &msg)
{
    size_t written = 0;
    ssize_t n = 0;

    while (written < msg.size())
    {
        n = ::write(fd, msg.data() + written, msg.size() - written);
        if (n == -1)
        {
            if (errno == EINTR || errno == EAGAIN)
                continue;
            break;
        }
        written += n;
    }

    return not (n == -1);
}

std::vector<uint8_t> Socket::read(size_t size)
{
	// malloc shouldnt be used so often but what the heck
    char *buffer = reinterpret_cast<char *>(malloc(size));

    int read_count = 0;
    while (read_count < size)
    {
        log.info("starting to read %d", read_count);
        int n = ::read(fd, buffer + read_count, size - read_count);
        log.info("read something (%d)", n);
		switch (n)
		{
			case -1:
				log.error("Read error on the connection using fd, %d", size);
				exit(1);
			case 0:
				log.debug("Received EOF (closed connection), %d", size);
				return {};
			default:
				read_count += n;
		}
    }

	std::vector<uint8_t> result (read_count);
    std::copy(buffer, buffer + static_cast<size_t>(read_count), result.begin());
	free(buffer);

	return result;
}
