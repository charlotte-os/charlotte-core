import clippy
import finder
import injector


def main():
    for lint in clippy.fetch():
        source_data = finder.find_usize_cast(lint)
        if source_data is None:
            continue
        injector.inject(*source_data)


if __name__ == '__main__':
    main()
