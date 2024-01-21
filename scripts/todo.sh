todo() {

    echo ""
    case "$1" in
        add)
            echo "Adding $2"
            ;;
        done)
            echo "Completed $2"
            ;;
        rm)
            echo "Removing $2"
            ;;
        list)
            echo "Listing"
            ;;
        sort)
            echo "Sorting"
            ;;
        *)
            echo "Unknown command: $1"
            exit 1
            ;;
    esac
}
